use std::{
    collections::HashMap,
    str::from_utf8,
    time::{Instant, SystemTime},
};

use cached::{Cached, SizedCache};
use chrono::NaiveDateTime;
use ergo_lib::ergotree_ir::{
    chain::{
        address::{Address, NetworkAddress, NetworkPrefix},
        ergo_box::RegisterValue,
    },
    ergo_tree::ErgoTree,
    mir::constant::TryExtractInto,
    serialization::SigmaSerializable,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait, Value,
};
use tokio::sync::mpsc::Receiver;
use tracing::info;

use ergo_node_client::apis::configuration::Configuration;

use crate::{
    actors::mempool_inserter::mempool_inserter, actors::mempool_listener::mempool_listener,
    actors::zmq_publisher::ZmqMessage, database::CIDatabase, entities, settings::Settings,
    sync_indexes::SavedIndexes, types::mempool_work::MempoolWork, types::work_object::WorkBlock,
};

/// Lightweight model for box cache-miss lookups (skips the large `data` JSON column)
#[derive(sea_orm::FromQueryResult)]
struct BoxCacheLookup {
    id: i64,
    box_id: String,
    address: i64,
    created: i32,
    spent: Option<i32>,
    transaction_id: Option<i64>,
}

const BATCH_SIZE: usize = 10;

pub async fn insert_data(
    mut receiver: Receiver<WorkBlock>,
    mempool_sender: Option<tokio::sync::mpsc::Sender<MempoolWork>>,
    zmq_sender: tokio::sync::mpsc::Sender<ZmqMessage>,
    saved_indexes: Option<SavedIndexes>,
    node_conf: Configuration,
    mempool_enabled: bool,
    mempool_resync_interval: i32,
    mempool_rx: Option<Receiver<MempoolWork>>,
    mempool_tx: tokio::sync::mpsc::Sender<MempoolWork>,
) {
    let settings = Settings::new().unwrap();
    let mut address_cached: SizedCache<String, entities::addresses::Model> =
        SizedCache::with_size(100000);
    let mut box_cached: SizedCache<String, entities::boxes::Model> = SizedCache::with_size(100000);
    let mut token_cached: SizedCache<String, entities::tokens::Model> =
        SizedCache::with_size(100000);
    let db = CIDatabase {
        settings: settings.to_owned(),
    }
    .connect()
    .await;

    let mut current_time = SystemTime::now();

    let mut current_box_id = match entities::boxes::Entity::find()
        .order_by_desc(entities::boxes::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };
    let mut current_transaction_id = match entities::transactions::Entity::find()
        .order_by_desc(entities::transactions::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };
    let mut current_token_id = match entities::tokens::Entity::find()
        .order_by_desc(entities::tokens::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };
    let mut current_token_in_box_id = match entities::token_in_box::Entity::find()
        .order_by_desc(entities::token_in_box::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };
    let mut current_address_id = match entities::addresses::Entity::find()
        .order_by_desc(entities::addresses::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };
    let mut current_input_id = match entities::inputs::Entity::find()
        .order_by_desc(entities::inputs::Column::Id)
        .one(&db)
        .await
    {
        Ok(Some(m)) => m.id,
        _ => 0,
    };

    let mut first_zmq: bool = true;
    let mut mempool_rx = mempool_rx;

    // Batch accumulation vectors
    let mut batch_blocks = Vec::<entities::blocks::ActiveModel>::new();
    let mut batch_boxes = Vec::<entities::boxes::ActiveModel>::new();
    let mut batch_spent_boxes: HashMap<i32, Vec<i64>> = HashMap::new(); // height -> box ids
    let mut batch_transactions = Vec::<entities::transactions::ActiveModel>::new();
    let mut batch_inputs = Vec::<entities::inputs::ActiveModel>::new();
    let mut batch_tokens = Vec::<entities::tokens::ActiveModel>::new();
    let mut batch_tokens_in_box = Vec::<entities::token_in_box::ActiveModel>::new();
    let mut batch_addresses = Vec::<entities::addresses::ActiveModel>::new();
    let mut batch_count: usize = 0;
    let mut batch_start = Instant::now();
    let mut batch_processing_time = std::time::Duration::ZERO;
    let mut batch_box_cache_misses: u32 = 0;
    let mut batch_addr_cache_misses: u32 = 0;
    let mut batch_token_cache_misses: u32 = 0;
    let mut batch_prefetch_time = std::time::Duration::ZERO;
    let mut batch_block_process_time = std::time::Duration::ZERO;
    let mut batch_total_txs: usize = 0;
    let mut batch_first_height: i32 = 0;
    let mut batch_last_height: i32 = 0;
    // For ZMQ mode notifications
    let mut batch_zmq_headers: Vec<(i32, String)> = Vec::new(); // (height, header_id)
    let mut batch_confirmed_tx_ids: Vec<(i32, Vec<String>)> = Vec::new(); // (height, tx_ids)

    while let Some(work_block) = receiver.recv().await {
        if work_block.zmq_mode && first_zmq {
            first_zmq = false;

            // Recreate indexes and foreign keys that were dropped during initial sync
            if let Some(ref saved) = saved_indexes {
                info!("Recreating indexes and foreign keys after initial sync");
                crate::sync_indexes::recreate_sync_indexes(&db, saved).await;
            }

            // Spawn mempool actors now that sync is complete
            if mempool_enabled {
                if let Some(rx) = mempool_rx.take() {
                    info!("Starting mempool tracking (sync complete)");
                    let mempool_db = CIDatabase {
                        settings: settings.to_owned(),
                    }
                    .connect()
                    .await;

                    let node_conf_inserter = node_conf.clone();
                    let zmq_sender_mempool = zmq_sender.clone();
                    let resync_interval = mempool_resync_interval;
                    tokio::spawn(async move {
                        mempool_inserter(
                            rx,
                            node_conf_inserter,
                            mempool_db,
                            resync_interval,
                            zmq_sender_mempool,
                        )
                        .await;
                    });

                    let node_conf_listener = node_conf.clone();
                    let mempool_tx_clone = mempool_tx.clone();
                    tokio::spawn(async move {
                        mempool_listener(mempool_tx_clone, node_conf_listener).await;
                    });
                }
            }
        }

        match work_block.rollback_height {
            Some(rollback_height) => {
                // Flush any pending batch before rollback (shouldn't normally have one, but safety)
                if batch_count > 0 {
                    flush_batch(
                        &db,
                        &mut batch_blocks,
                        &mut batch_addresses,
                        &mut batch_transactions,
                        &mut batch_boxes,
                        &mut batch_spent_boxes,
                        &mut batch_tokens,
                        &mut batch_tokens_in_box,
                        &mut batch_inputs,
                        &mut batch_count,
                        &mut batch_start,
                        &mut batch_processing_time,
                        &mut batch_prefetch_time,
                        &mut batch_block_process_time,
                        &mut batch_box_cache_misses,
                        &mut batch_addr_cache_misses,
                        &mut batch_token_cache_misses,
                        &mut batch_total_txs,
                        &mut batch_first_height,
                        &mut batch_last_height,
                        &mut batch_zmq_headers,
                        &mut batch_confirmed_tx_ids,
                        &zmq_sender,
                        &mempool_sender,
                    )
                    .await;
                }

                info!("Processing rollback to height: {}", rollback_height);
                match entities::blocks::Entity::delete_many()
                    .filter(entities::blocks::Column::Height.gt(rollback_height))
                    .exec(&db)
                    .await
                {
                    Ok(result) => info!("Rollback deleted {} blocks", result.rows_affected),
                    Err(e) => panic!(
                        "Failed to execute rollback to height {}: {}",
                        rollback_height, e
                    ),
                };
                address_cached.cache_clear();
                box_cached.cache_clear();
                token_cached.cache_clear();
                current_box_id = match entities::boxes::Entity::find()
                    .order_by_desc(entities::boxes::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
                current_transaction_id = match entities::transactions::Entity::find()
                    .order_by_desc(entities::transactions::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
                current_token_id = match entities::tokens::Entity::find()
                    .order_by_desc(entities::tokens::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
                current_token_in_box_id = match entities::token_in_box::Entity::find()
                    .order_by_desc(entities::token_in_box::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
                current_address_id = match entities::addresses::Entity::find()
                    .order_by_desc(entities::addresses::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
                current_input_id = match entities::inputs::Entity::find()
                    .order_by_desc(entities::inputs::Column::Id)
                    .one(&db)
                    .await
                    .unwrap()
                {
                    Some(m) => m.id,
                    None => 0,
                };
            }
            None => {
                let processing_start = Instant::now();

                if batch_count == 0 {
                    batch_start = Instant::now();
                }

                let header = work_block.header.unwrap();
                let transactions = work_block.transactions.unwrap();
                let is_zmq = work_block.zmq_mode;

                if batch_count == 0 {
                    batch_first_height = header.height;
                }
                batch_last_height = header.height;

                // Insert block record
                let block = entities::blocks::ActiveModel {
                    height: Set(header.height),
                    header_id: Set(header.id.to_string()),
                    timestamp: Set(NaiveDateTime::from_timestamp_millis(header.timestamp)
                        .expect("Expected NaiveDateTime")),
                    header: Set(serde_json::json!(header)),
                };
                batch_blocks.push(block);

                // Pre-pass: collect all cache-missing IDs for batch lookup
                let prefetch_start = Instant::now();
                let mut missing_box_ids = Vec::<String>::new();
                let mut missing_ergotrees = Vec::<String>::new();
                let mut missing_token_ids = Vec::<String>::new();

                for ergotx in transactions.iter() {
                    for input in ergotx.inputs.iter() {
                        if box_cached.cache_get(&input.box_id).is_none() {
                            missing_box_ids.push(input.box_id.clone());
                        }
                    }
                    for outp in ergotx.outputs.iter() {
                        if address_cached.cache_get(&outp.ergo_tree).is_none() {
                            missing_ergotrees.push(outp.ergo_tree.clone());
                        }
                        if let Some(assets) = &outp.assets {
                            for ass in assets.iter() {
                                if token_cached.cache_get(&ass.token_id).is_none() {
                                    missing_token_ids.push(ass.token_id.clone());
                                }
                            }
                        }
                    }
                }

                // Deduplicate
                missing_box_ids.sort();
                missing_box_ids.dedup();
                missing_ergotrees.sort();
                missing_ergotrees.dedup();
                missing_token_ids.sort();
                missing_token_ids.dedup();

                // Batch fetch missing boxes (skip data column for performance - 17GB+ table)
                if !missing_box_ids.is_empty() {
                    batch_box_cache_misses += missing_box_ids.len() as u32;
                    for chunk in missing_box_ids.chunks(500) {
                        let results = entities::boxes::Entity::find()
                            .select_only()
                            .column(entities::boxes::Column::Id)
                            .column(entities::boxes::Column::BoxId)
                            .column(entities::boxes::Column::Address)
                            .column(entities::boxes::Column::Created)
                            .column(entities::boxes::Column::Spent)
                            .column(entities::boxes::Column::TransactionId)
                            .filter(entities::boxes::Column::BoxId.is_in(chunk.iter().cloned()))
                            .into_model::<BoxCacheLookup>()
                            .all(&db)
                            .await
                            .unwrap();
                        for r in results {
                            let m = entities::boxes::Model {
                                id: r.id,
                                box_id: r.box_id,
                                address: r.address,
                                created: r.created,
                                spent: r.spent,
                                transaction_id: r.transaction_id,
                                data: serde_json::json!(null),
                            };
                            box_cached.cache_set(m.box_id.clone(), m);
                        }
                    }
                }

                // Batch fetch missing addresses
                if !missing_ergotrees.is_empty() {
                    batch_addr_cache_misses += missing_ergotrees.len() as u32;
                    for chunk in missing_ergotrees.chunks(500) {
                        let results = entities::addresses::Entity::find()
                            .filter(
                                entities::addresses::Column::Ergotree.is_in(chunk.iter().cloned()),
                            )
                            .all(&db)
                            .await
                            .unwrap();
                        for m in results {
                            address_cached.cache_set(m.ergotree.clone(), m);
                        }
                    }
                }

                // Batch fetch missing tokens
                if !missing_token_ids.is_empty() {
                    batch_token_cache_misses += missing_token_ids.len() as u32;
                    for chunk in missing_token_ids.chunks(500) {
                        let results = entities::tokens::Entity::find()
                            .filter(entities::tokens::Column::TokenId.is_in(chunk.iter().cloned()))
                            .all(&db)
                            .await
                            .unwrap();
                        for m in results {
                            token_cached.cache_set(m.token_id.clone(), m);
                        }
                    }
                }

                let prefetch_elapsed = prefetch_start.elapsed();
                let block_process_start = Instant::now();
                let mut confirmed_tx_ids = Vec::<String>::new();

                for ergotx in transactions.iter() {
                    current_transaction_id += 1;
                    let tx_id_str = ergotx.id.to_owned().unwrap();
                    confirmed_tx_ids.push(tx_id_str.clone());
                    let transaction = entities::transactions::Model {
                        transaction_id: tx_id_str,
                        height: header.height,
                        id: current_transaction_id,
                    };
                    batch_transactions.push(transaction.to_owned().into_active_model());
                    let mut input_index = 0;
                    for input in ergotx.inputs.iter() {
                        current_input_id += 1;
                        let utxo = box_cached
                            .cache_get(&input.box_id)
                            .expect(&format!(
                                "Box {} not found in cache after batch prefetch",
                                input.box_id
                            ))
                            .to_owned();
                        let spent_txo = entities::boxes::Model {
                            spent: Some(header.height),
                            ..utxo
                        };
                        batch_spent_boxes
                            .entry(header.height)
                            .or_insert_with(Vec::new)
                            .push(spent_txo.id.to_owned());
                        box_cached.cache_set(input.box_id.to_owned(), spent_txo.to_owned());
                        let inp = entities::inputs::Model {
                            id: current_input_id,
                            box_id: spent_txo.id,
                            transaction_id: transaction.id,
                            spending_proof: serde_json::json!(input.spending_proof),
                            index: input_index,
                        };
                        batch_inputs.push(inp.into_active_model());
                        input_index += 1;
                    }
                    for outp in ergotx.outputs.iter() {
                        let address = match address_cached.cache_get(&outp.ergo_tree) {
                            Some(m) => m.to_owned(),
                            None => {
                                // Not in DB either — create new address
                                current_address_id += 1;
                                let new_address = entities::addresses::Model {
                                    ergotree: outp.ergo_tree.to_owned(),
                                    ergotree_template: Some(hex::encode(
                                        &ErgoTree::sigma_parse_bytes(
                                            &hex::decode(outp.ergo_tree.to_owned()).unwrap(),
                                        )
                                        .unwrap()
                                        .template_bytes()
                                        .unwrap_or(Vec::<u8>::new()),
                                    )),
                                    ergotree_hash: "".to_owned(),
                                    address: NetworkAddress::new(
                                        NetworkPrefix::Mainnet,
                                        &Address::recreate_from_ergo_tree(
                                            &ErgoTree::sigma_parse_bytes(
                                                &hex::decode(outp.ergo_tree.to_owned()).unwrap(),
                                            )
                                            .unwrap(),
                                        )
                                        .unwrap(),
                                    )
                                    .to_base58(),
                                    id: current_address_id,
                                };
                                batch_addresses.push(new_address.to_owned().into_active_model());
                                address_cached.cache_set(
                                    new_address.ergotree.to_owned(),
                                    new_address.to_owned(),
                                );
                                new_address
                            }
                        };
                        current_box_id += 1;
                        let new_box = entities::boxes::Model {
                            address: address.id,
                            created: header.height,
                            spent: None,
                            box_id: outp.box_id.to_owned().unwrap(),
                            transaction_id: Some(transaction.id),
                            data: serde_json::json!(outp),
                            id: current_box_id,
                        };
                        batch_boxes.push(new_box.to_owned().into_active_model());
                        box_cached.cache_set(new_box.box_id.to_owned(), new_box.to_owned());
                        let mut asset_index = 0;
                        for ass in outp.assets.to_owned().unwrap().iter() {
                            current_token_in_box_id += 1;
                            let token = match token_cached.cache_get(&ass.token_id) {
                                Some(t) => t.to_owned(),
                                None => {
                                    // Not in DB — new token being minted
                                    current_token_id += 1;
                                    let token_name = match outp.additional_registers.get("R4") {
                                        Some(serialized) => {
                                            let r_val = RegisterValue::sigma_parse_bytes(
                                                &hex::decode(&serialized).unwrap(),
                                            );
                                            match r_val
                                                .as_constant()
                                                .unwrap()
                                                .v
                                                .to_owned()
                                                .try_extract_into::<Vec<u8>>()
                                            {
                                                Ok(r) => from_utf8(r.as_slice())
                                                    .unwrap_or("")
                                                    .to_owned()
                                                    .replace("\u{00}", ""),
                                                Err(_error) => "".to_owned(),
                                            }
                                        }
                                        None => "".to_owned(),
                                    };
                                    let token_description =
                                        match outp.additional_registers.get("R5") {
                                            Some(serialized) => {
                                                let r_val = RegisterValue::sigma_parse_bytes(
                                                    &hex::decode(&serialized).unwrap(),
                                                );
                                                match r_val
                                                    .as_constant()
                                                    .unwrap()
                                                    .v
                                                    .to_owned()
                                                    .try_extract_into::<Vec<u8>>()
                                                {
                                                    Ok(r) => from_utf8(r.as_slice())
                                                        .unwrap_or("")
                                                        .to_owned()
                                                        .replace("\u{00}", ""),
                                                    Err(_error) => "".to_owned(),
                                                }
                                            }
                                            None => "".to_owned(),
                                        };
                                    let token_decimals: i32 =
                                        match outp.additional_registers.get("R6") {
                                            Some(serialized) => {
                                                let r_val = RegisterValue::sigma_parse_bytes(
                                                    &hex::decode(&serialized).unwrap(),
                                                );
                                                match r_val
                                                    .as_constant()
                                                    .unwrap()
                                                    .v
                                                    .to_owned()
                                                    .try_extract_into::<Vec<u8>>()
                                                {
                                                    Ok(r) => {
                                                        match from_utf8(r.as_slice())
                                                            .unwrap_or("")
                                                            .parse::<i32>()
                                                        {
                                                            Ok(i) => i,
                                                            Err(_error) => 0,
                                                        }
                                                    }
                                                    Err(_error) => 0,
                                                }
                                            }
                                            None => 0,
                                        };
                                    let new_token = entities::tokens::Model {
                                        token_id: ass.token_id.to_owned(),
                                        token_name: token_name,
                                        token_description: token_description,
                                        decimals: Some(token_decimals),
                                        issuer_box: box_cached
                                            .cache_get(&ergotx.inputs[0].box_id)
                                            .to_owned()
                                            .unwrap()
                                            .id,
                                        issuance_box: new_box.id.to_owned(),
                                        minted: Some(ass.amount.to_owned()),
                                        id: current_token_id,
                                    };
                                    batch_tokens.push(new_token.to_owned().into_active_model());
                                    token_cached.cache_set(
                                        new_token.token_id.to_owned(),
                                        new_token.to_owned(),
                                    );
                                    new_token
                                }
                            };

                            let new_token_in_box = entities::token_in_box::Model {
                                box_id: new_box.id,
                                token_id: token.id,
                                amount: ass.amount,
                                index: asset_index,
                                id: current_token_in_box_id,
                            };

                            batch_tokens_in_box.push(new_token_in_box.into_active_model());
                            asset_index += 1;
                        }
                    }
                }

                batch_total_txs += transactions.len();
                batch_prefetch_time += prefetch_elapsed;
                batch_block_process_time += block_process_start.elapsed();
                batch_processing_time += processing_start.elapsed();
                batch_count += 1;

                if is_zmq {
                    batch_zmq_headers.push((header.height, header.id.to_string()));
                    batch_confirmed_tx_ids.push((header.height, confirmed_tx_ids));
                }

                if header.height % 1000 == 0 {
                    let time_now = SystemTime::now();
                    let time_spent = time_now.duration_since(current_time).unwrap();
                    current_time = time_now;
                    info!(
                        "DB Height: {}, speed: {}",
                        header.height,
                        (1000f32 / time_spent.as_secs_f32())
                    );
                }

                // Flush batch when full, or always in ZMQ mode
                let effective_batch_size = if is_zmq { 1 } else { BATCH_SIZE };
                if batch_count >= effective_batch_size {
                    flush_batch(
                        &db,
                        &mut batch_blocks,
                        &mut batch_addresses,
                        &mut batch_transactions,
                        &mut batch_boxes,
                        &mut batch_spent_boxes,
                        &mut batch_tokens,
                        &mut batch_tokens_in_box,
                        &mut batch_inputs,
                        &mut batch_count,
                        &mut batch_start,
                        &mut batch_processing_time,
                        &mut batch_prefetch_time,
                        &mut batch_block_process_time,
                        &mut batch_box_cache_misses,
                        &mut batch_addr_cache_misses,
                        &mut batch_token_cache_misses,
                        &mut batch_total_txs,
                        &mut batch_first_height,
                        &mut batch_last_height,
                        &mut batch_zmq_headers,
                        &mut batch_confirmed_tx_ids,
                        &zmq_sender,
                        &mempool_sender,
                    )
                    .await;
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn flush_batch(
    db: &sea_orm::DatabaseConnection,
    batch_blocks: &mut Vec<entities::blocks::ActiveModel>,
    batch_addresses: &mut Vec<entities::addresses::ActiveModel>,
    batch_transactions: &mut Vec<entities::transactions::ActiveModel>,
    batch_boxes: &mut Vec<entities::boxes::ActiveModel>,
    batch_spent_boxes: &mut HashMap<i32, Vec<i64>>,
    batch_tokens: &mut Vec<entities::tokens::ActiveModel>,
    batch_tokens_in_box: &mut Vec<entities::token_in_box::ActiveModel>,
    batch_inputs: &mut Vec<entities::inputs::ActiveModel>,
    batch_count: &mut usize,
    batch_start: &mut Instant,
    batch_processing_time: &mut std::time::Duration,
    batch_prefetch_time: &mut std::time::Duration,
    batch_block_process_time: &mut std::time::Duration,
    batch_box_cache_misses: &mut u32,
    batch_addr_cache_misses: &mut u32,
    batch_token_cache_misses: &mut u32,
    batch_total_txs: &mut usize,
    batch_first_height: &mut i32,
    batch_last_height: &mut i32,
    batch_zmq_headers: &mut Vec<(i32, String)>,
    batch_confirmed_tx_ids: &mut Vec<(i32, Vec<String>)>,
    zmq_sender: &tokio::sync::mpsc::Sender<ZmqMessage>,
    mempool_sender: &Option<tokio::sync::mpsc::Sender<MempoolWork>>,
) {
    if *batch_count == 0 {
        return;
    }

    let db_insert_start = Instant::now();
    let tx = db.begin().await.unwrap();

    // Insert blocks
    let t0 = Instant::now();
    for block in batch_blocks.drain(..) {
        match block.insert(&tx).await {
            Ok(_) => (),
            Err(e) => panic!("Failed to insert block: {}", e),
        };
    }
    let t_blocks = t0.elapsed();

    // Insert addresses
    let t0 = Instant::now();
    if !batch_addresses.is_empty() {
        for chunk in batch_addresses.as_slice().chunks(1000) {
            match entities::addresses::Entity::insert_many(chunk.to_owned())
                .exec(&tx)
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    }
    let t_addresses = t0.elapsed();

    // Insert transactions
    let t0 = Instant::now();
    for chunk in batch_transactions.as_slice().chunks(5000) {
        match entities::transactions::Entity::insert_many(chunk.to_owned())
            .exec(&tx)
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }
    let t_transactions = t0.elapsed();

    // Insert boxes
    let t0 = Instant::now();
    let num_boxes = batch_boxes.len();
    for chunk in batch_boxes.as_slice().chunks(1000) {
        match entities::boxes::Entity::insert_many(chunk.to_owned())
            .exec(&tx)
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }
    let t_boxes = t0.elapsed();

    // Update spent boxes per height
    let t0 = Instant::now();
    let num_spent = batch_spent_boxes.values().map(|v| v.len()).sum::<usize>();
    for (height, box_ids) in batch_spent_boxes.drain() {
        for chunk in box_ids.chunks(5000) {
            match entities::boxes::Entity::update_many()
                .col_expr(
                    entities::boxes::Column::Spent,
                    Expr::value(Value::Int(Some(height))),
                )
                .filter(entities::boxes::Column::Id.is_in(chunk.iter().copied()))
                .exec(&tx)
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    }
    let t_spent = t0.elapsed();

    // Insert tokens
    let t0 = Instant::now();
    if !batch_tokens.is_empty() {
        for chunk in batch_tokens.as_slice().chunks(1000) {
            match entities::tokens::Entity::insert_many(chunk.to_owned())
                .exec(&tx)
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    }
    let t_tokens = t0.elapsed();

    // Insert token_in_box
    let t0 = Instant::now();
    let num_tib = batch_tokens_in_box.len();
    if !batch_tokens_in_box.is_empty() {
        for chunk in batch_tokens_in_box.as_slice().chunks(1000) {
            match entities::token_in_box::Entity::insert_many(chunk.to_owned())
                .exec(&tx)
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    }
    let t_tib = t0.elapsed();

    // Insert inputs
    let t0 = Instant::now();
    let num_inputs = batch_inputs.len();
    if !batch_inputs.is_empty() {
        for chunk in batch_inputs.as_slice().chunks(2000) {
            match entities::inputs::Entity::insert_many(chunk.to_owned())
                .exec(&tx)
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
        }
    }
    let t_inputs = t0.elapsed();

    let db_insert_elapsed = db_insert_start.elapsed();
    let commit_start = Instant::now();
    let _ = tx.commit().await;
    let commit_elapsed = commit_start.elapsed();
    let batch_elapsed = batch_start.elapsed();

    let num_blocks = *batch_count;

    info!(
        "Batch {}-{} ({} blocks) | total: {:?} | prefetch: {:?} | block_proc: {:?} | db_inserts: {:?} | commit: {:?} | txs: {} | boxes: {} | inputs: {} | spent: {} | tib: {} | cache_misses(box/addr/tok): {}/{}/{} | insert_times(blk/addr/tx/box/spent/tok/tib/inp): {:?}/{:?}/{:?}/{:?}/{:?}/{:?}/{:?}/{:?}",
        batch_first_height,
        batch_last_height,
        num_blocks,
        batch_elapsed,
        batch_prefetch_time,
        batch_block_process_time,
        db_insert_elapsed,
        commit_elapsed,
        batch_total_txs,
        num_boxes,
        num_inputs,
        num_spent,
        num_tib,
        batch_box_cache_misses,
        batch_addr_cache_misses,
        batch_token_cache_misses,
        t_blocks, t_addresses, t_transactions, t_boxes, t_spent, t_tokens, t_tib, t_inputs,
    );

    // Send ZMQ notifications for all blocks in batch
    for (height, header_id) in batch_zmq_headers.drain(..) {
        info!(
            "Inserted block with height: {} and header: {}",
            height, header_id
        );
        let _ = zmq_sender.send(("new_block".to_string(), header_id)).await;
    }
    if let Some(ref mp_sender) = mempool_sender {
        for (block_height, confirmed_tx_ids) in batch_confirmed_tx_ids.drain(..) {
            let _ = mp_sender
                .send(MempoolWork::BlockConfirmed {
                    block_height,
                    confirmed_tx_ids,
                })
                .await;
        }
    }

    // Reset batch state
    batch_blocks.clear();
    batch_addresses.clear();
    batch_transactions.clear();
    batch_boxes.clear();
    batch_tokens.clear();
    batch_tokens_in_box.clear();
    batch_inputs.clear();
    *batch_count = 0;
    *batch_processing_time = std::time::Duration::ZERO;
    *batch_prefetch_time = std::time::Duration::ZERO;
    *batch_block_process_time = std::time::Duration::ZERO;
    *batch_box_cache_misses = 0;
    *batch_addr_cache_misses = 0;
    *batch_token_cache_misses = 0;
    *batch_total_txs = 0;
    *batch_first_height = 0;
    *batch_last_height = 0;
}
