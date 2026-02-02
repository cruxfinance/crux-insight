use std::str::from_utf8;

use chrono::Utc;
use ergo_lib::ergotree_ir::{
    chain::ergo_box::RegisterValue, mir::constant::TryExtractInto, serialization::SigmaSerializable,
};
use ergo_node_client::{
    apis::{configuration::Configuration, transactions_api},
    models::ErgoTransaction,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use std::collections::HashSet;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info, warn};

use crate::{
    actors::zmq_publisher::ZmqMessage,
    entities::{
        self, mempool_boxes, mempool_inputs, mempool_token_in_box, mempool_tokens,
        mempool_transactions,
    },
    types::mempool_work::MempoolWork,
};

/// Inserts a single mempool transaction into the database
async fn insert_mempool_transaction(
    db: &DatabaseConnection,
    tx: &ErgoTransaction,
) -> Result<(), sea_orm::DbErr> {
    let tx_id = tx
        .id
        .as_ref()
        .ok_or_else(|| sea_orm::DbErr::Custom("Transaction has no ID".to_string()))?;

    // Check if transaction already exists
    let existing = mempool_transactions::Entity::find()
        .filter(mempool_transactions::Column::TransactionId.eq(tx_id))
        .one(db)
        .await?;

    if existing.is_some() {
        debug!("Mempool transaction {} already exists, skipping", tx_id);
        return Ok(());
    }

    let db_tx = db.begin().await?;

    // Insert the transaction
    let mempool_tx = mempool_transactions::ActiveModel {
        transaction_id: Set(tx_id.clone()),
        data: Set(serde_json::json!(tx)),
        first_seen: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let inserted_tx = mempool_tx.insert(&db_tx).await?;

    // Track boxes we create for token issuance lookups
    let mut new_box_ids: Vec<(String, i64)> = Vec::new();

    // Process outputs (boxes)
    for (output_index, output) in tx.outputs.iter().enumerate() {
        let box_id = output
            .box_id
            .as_ref()
            .ok_or_else(|| sea_orm::DbErr::Custom("Output has no box_id".to_string()))?;

        // Resolve address if it exists (may be None if not yet synced)
        let address_id = resolve_address(&db_tx, &output.ergo_tree).await?;

        let mempool_box = mempool_boxes::ActiveModel {
            box_id: Set(box_id.clone()),
            address: Set(address_id),
            ergotree: Set(output.ergo_tree.clone()),
            data: Set(serde_json::json!(output)),
            mempool_transaction_id: Set(inserted_tx.id),
            output_index: Set(output_index as i32),
            ..Default::default()
        };
        let inserted_box = mempool_box.insert(&db_tx).await?;
        new_box_ids.push((box_id.clone(), inserted_box.id));

        // Process tokens in this box
        if let Some(assets) = &output.assets {
            for (asset_index, asset) in assets.iter().enumerate() {
                // Check if this is an existing token or a new one being minted
                let existing_token = entities::tokens::Entity::find()
                    .filter(entities::tokens::Column::TokenId.eq(&asset.token_id))
                    .one(&db_tx)
                    .await?;

                let (token_id, mempool_token_id) = if let Some(token) = existing_token {
                    (Some(token.id), None)
                } else {
                    // Check if it's a mempool token we already created
                    let existing_mempool_token = mempool_tokens::Entity::find()
                        .filter(mempool_tokens::Column::TokenId.eq(&asset.token_id))
                        .one(&db_tx)
                        .await?;

                    if let Some(mp_token) = existing_mempool_token {
                        (None, Some(mp_token.id))
                    } else {
                        // This is a new token being minted - check if this box is the issuance box
                        // Token ID equals the first input's box_id
                        let first_input_box_id = tx.inputs.first().map(|i| &i.box_id);
                        if first_input_box_id == Some(&asset.token_id) {
                            // This is a new token issuance
                            let (token_name, token_desc, decimals) =
                                extract_token_metadata(&output.additional_registers);

                            let new_mempool_token = mempool_tokens::ActiveModel {
                                token_id: Set(asset.token_id.clone()),
                                token_name: Set(token_name),
                                token_description: Set(token_desc),
                                decimals: Set(decimals),
                                minted: Set(Some(asset.amount)),
                                issuer_box: Set(asset.token_id.clone()),
                                issuance_box: Set(inserted_box.id),
                                ..Default::default()
                            };
                            let inserted_mp_token = new_mempool_token.insert(&db_tx).await?;
                            (None, Some(inserted_mp_token.id))
                        } else {
                            // Unknown token - still track it but without proper references
                            warn!(
                                "Unknown token {} in mempool tx {}, skipping token_in_box entry",
                                asset.token_id, tx_id
                            );
                            continue;
                        }
                    }
                };

                let token_in_box = mempool_token_in_box::ActiveModel {
                    mempool_box_id: Set(inserted_box.id),
                    token_id: Set(token_id),
                    mempool_token_id: Set(mempool_token_id),
                    token_id_str: Set(asset.token_id.clone()),
                    amount: Set(asset.amount),
                    index: Set(asset_index as i32),
                    ..Default::default()
                };
                token_in_box.insert(&db_tx).await?;
            }
        }
    }

    // Process inputs
    for (input_index, input) in tx.inputs.iter().enumerate() {
        // Try to find the confirmed box being spent
        let confirmed_box_id = resolve_confirmed_box_id(&db_tx, &input.box_id).await?;

        let mempool_input = mempool_inputs::ActiveModel {
            mempool_transaction_id: Set(inserted_tx.id),
            box_id: Set(input.box_id.clone()),
            confirmed_box_id: Set(confirmed_box_id),
            spending_proof: Set(serde_json::json!(input.spending_proof)),
            index: Set(input_index as i32),
            ..Default::default()
        };
        mempool_input.insert(&db_tx).await?;
    }

    db_tx.commit().await?;
    info!("Inserted mempool transaction: {}", tx_id);
    Ok(())
}

/// Resolve an ergotree to an address ID (returns None if address doesn't exist)
async fn resolve_address(
    db: &impl sea_orm::ConnectionTrait,
    ergotree: &str,
) -> Result<Option<i64>, sea_orm::DbErr> {
    let existing = entities::addresses::Entity::find()
        .filter(entities::addresses::Column::Ergotree.eq(ergotree))
        .one(db)
        .await?;

    Ok(existing.map(|addr| addr.id))
}

/// Try to find a confirmed box by its box_id
async fn resolve_confirmed_box_id(
    db: &impl sea_orm::ConnectionTrait,
    box_id: &str,
) -> Result<Option<i64>, sea_orm::DbErr> {
    let existing = entities::boxes::Entity::find()
        .filter(entities::boxes::Column::BoxId.eq(box_id))
        .one(db)
        .await?;

    Ok(existing.map(|b| b.id))
}

/// Extract token metadata from registers (R4=name, R5=description, R6=decimals)
fn extract_token_metadata(
    registers: &std::collections::HashMap<String, String>,
) -> (Option<String>, Option<String>, Option<i32>) {
    let token_name = registers.get("R4").and_then(|serialized| {
        let r_val = RegisterValue::sigma_parse_bytes(&hex::decode(serialized).ok()?);
        r_val
            .as_constant()
            .ok()?
            .v
            .clone()
            .try_extract_into::<Vec<u8>>()
            .ok()
            .and_then(|r| {
                from_utf8(r.as_slice())
                    .ok()
                    .map(|s| s.replace("\u{00}", ""))
            })
    });

    let token_desc = registers.get("R5").and_then(|serialized| {
        let r_val = RegisterValue::sigma_parse_bytes(&hex::decode(serialized).ok()?);
        r_val
            .as_constant()
            .ok()?
            .v
            .clone()
            .try_extract_into::<Vec<u8>>()
            .ok()
            .and_then(|r| {
                from_utf8(r.as_slice())
                    .ok()
                    .map(|s| s.replace("\u{00}", ""))
            })
    });

    let decimals = registers.get("R6").and_then(|serialized| {
        let r_val = RegisterValue::sigma_parse_bytes(&hex::decode(serialized).ok()?);
        r_val
            .as_constant()
            .ok()?
            .v
            .clone()
            .try_extract_into::<Vec<u8>>()
            .ok()
            .and_then(|r| from_utf8(r.as_slice()).ok()?.parse::<i32>().ok())
    });

    (token_name, token_desc, decimals)
}

/// Handle block confirmation - remove confirmed and invalidated transactions
async fn handle_block_confirmed(
    db: &DatabaseConnection,
    confirmed_tx_ids: Vec<String>,
) -> Result<(), sea_orm::DbErr> {
    let db_tx = db.begin().await?;

    // 1. Delete confirmed transactions from mempool (CASCADE handles related records)
    if !confirmed_tx_ids.is_empty() {
        let delete_result = mempool_transactions::Entity::delete_many()
            .filter(mempool_transactions::Column::TransactionId.is_in(confirmed_tx_ids.clone()))
            .exec(&db_tx)
            .await?;
        info!(
            "Removed {} confirmed transactions from mempool",
            delete_result.rows_affected
        );
    }

    // 2. Find and delete invalidated transactions (those spending now-spent boxes)
    // Get all box_ids referenced by mempool inputs
    let mempool_input_box_ids: Vec<String> = mempool_inputs::Entity::find()
        .all(&db_tx)
        .await?
        .into_iter()
        .map(|i| i.box_id)
        .collect();

    if !mempool_input_box_ids.is_empty() {
        // Find which of these boxes are now spent (confirmed)
        let spent_boxes: Vec<String> = entities::boxes::Entity::find()
            .filter(entities::boxes::Column::BoxId.is_in(mempool_input_box_ids))
            .filter(entities::boxes::Column::Spent.is_not_null())
            .all(&db_tx)
            .await?
            .into_iter()
            .map(|b| b.box_id)
            .collect();

        if !spent_boxes.is_empty() {
            // Find mempool transactions that reference these spent boxes
            let invalidated_tx_ids: Vec<i64> = mempool_inputs::Entity::find()
                .filter(mempool_inputs::Column::BoxId.is_in(spent_boxes))
                .all(&db_tx)
                .await?
                .into_iter()
                .map(|i| i.mempool_transaction_id)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            if !invalidated_tx_ids.is_empty() {
                let delete_result = mempool_transactions::Entity::delete_many()
                    .filter(mempool_transactions::Column::Id.is_in(invalidated_tx_ids))
                    .exec(&db_tx)
                    .await?;
                info!(
                    "Removed {} invalidated transactions from mempool",
                    delete_result.rows_affected
                );
            }
        }
    }

    db_tx.commit().await?;
    Ok(())
}

/// Perform a full mempool resync with the node
async fn full_mempool_resync(
    node_conf: &Configuration,
    db: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting full mempool resync");

    // 1. Fetch all current mempool tx IDs from node
    let mut all_mempool_txs = Vec::new();
    let mut offset = 0;
    let limit = 100;
    loop {
        let txs =
            transactions_api::get_unconfirmed_transactions(node_conf, Some(limit), Some(offset))
                .await?;
        if txs.is_empty() {
            break;
        }
        all_mempool_txs.extend(txs);
        offset += limit;
    }

    let node_tx_ids: HashSet<String> = all_mempool_txs
        .iter()
        .filter_map(|tx| tx.id.clone())
        .collect();

    // 2. Get all tx IDs currently in our mempool tables
    let db_txs = mempool_transactions::Entity::find().all(db).await?;
    let db_tx_ids: HashSet<String> = db_txs.into_iter().map(|t| t.transaction_id).collect();

    // 3. Remove transactions that are no longer in node's mempool
    let to_remove: Vec<String> = db_tx_ids.difference(&node_tx_ids).cloned().collect();
    if !to_remove.is_empty() {
        let delete_result = mempool_transactions::Entity::delete_many()
            .filter(mempool_transactions::Column::TransactionId.is_in(to_remove.clone()))
            .exec(db)
            .await?;
        info!(
            "Resync removed {} stale transactions from mempool",
            delete_result.rows_affected
        );
    }

    // 4. Add transactions that are in node but not in our DB
    let to_add: Vec<&ErgoTransaction> = all_mempool_txs
        .iter()
        .filter(|tx| {
            tx.id
                .as_ref()
                .map(|id| !db_tx_ids.contains(id))
                .unwrap_or(false)
        })
        .collect();

    for tx in to_add {
        if let Err(e) = insert_mempool_transaction(db, tx).await {
            error!("Failed to insert mempool tx during resync: {}", e);
        }
    }

    info!("Full mempool resync complete");
    Ok(())
}

/// Main mempool inserter actor function
pub async fn mempool_inserter(
    mut receiver: Receiver<MempoolWork>,
    node_conf: Configuration,
    db: DatabaseConnection,
    resync_interval: i32,
    zmq_sender: tokio::sync::mpsc::Sender<ZmqMessage>,
) {
    let mut blocks_since_resync = 0;

    while let Some(work) = receiver.recv().await {
        match work {
            MempoolWork::NewTransaction(tx) => {
                let tx_id = tx.id.clone().unwrap_or_default();
                if let Err(e) = insert_mempool_transaction(&db, &tx).await {
                    error!("Failed to insert mempool transaction: {}", e);
                } else {
                    let _ = zmq_sender.send(("mempool_tx".to_string(), tx_id)).await;
                }
            }
            MempoolWork::BlockConfirmed {
                block_height,
                confirmed_tx_ids,
            } => {
                debug!("Block {} confirmed, cleaning up mempool", block_height);
                if let Err(e) = handle_block_confirmed(&db, confirmed_tx_ids).await {
                    error!("Failed to handle block confirmation: {}", e);
                }
                let _ = zmq_sender
                    .send(("mempool_sync".to_string(), String::new()))
                    .await;

                // Check if we need a full resync
                blocks_since_resync += 1;
                if blocks_since_resync >= resync_interval {
                    blocks_since_resync = 0;
                    if let Err(e) = full_mempool_resync(&node_conf, &db).await {
                        error!("Failed to perform full mempool resync: {}", e);
                    }
                    let _ = zmq_sender
                        .send(("mempool_sync".to_string(), String::new()))
                        .await;
                }
            }
            MempoolWork::FullResync => {
                blocks_since_resync = 0;
                if let Err(e) = full_mempool_resync(&node_conf, &db).await {
                    error!("Failed to perform full mempool resync: {}", e);
                }
                let _ = zmq_sender
                    .send(("mempool_sync".to_string(), String::new()))
                    .await;
            }
        }
    }
}
