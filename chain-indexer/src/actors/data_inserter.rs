use std::{str::from_utf8, time::SystemTime};

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
use futures::SinkExt;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set, TransactionTrait, Value,
};
use tmq::{publish, Context};
use tokio::sync::mpsc::Receiver;
use tracing::info;

use crate::{database::CIDatabase, entities, settings::Settings, types::work_object::WorkBlock};

pub async fn insert_data(mut receiver: Receiver<WorkBlock>) {
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

    let mut socket = publish(&Context::new())
        .bind(&format!("tcp://0.0.0.0:{}", &settings.crux.pubsubport))
        .unwrap();

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

    while let Some(work_block) = receiver.recv().await {
        match work_block.rollback_height {
            Some(rollback_height) => {
                let _ = entities::blocks::Entity::delete_many()
                    .filter(entities::blocks::Column::Height.gt(rollback_height))
                    .exec(&db)
                    .await;
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
                let mut new_boxes = Vec::<entities::boxes::ActiveModel>::new();
                let mut updated_boxes = Vec::<i64>::new();
                let mut new_transactions = Vec::<entities::transactions::ActiveModel>::new();
                let mut new_inputs = Vec::<entities::inputs::ActiveModel>::new();
                let mut new_tokens = Vec::<entities::tokens::ActiveModel>::new();
                let mut new_tokens_in_box = Vec::<entities::token_in_box::ActiveModel>::new();
                let mut new_addresses = Vec::<entities::addresses::ActiveModel>::new();

                let header = work_block.header.unwrap();
                let transactions = work_block.transactions.unwrap();

                let tx = db.begin().await.unwrap();
                let block = entities::blocks::ActiveModel {
                    height: Set(header.height),
                    header_id: Set(header.id.to_string()),
                    timestamp: Set(NaiveDateTime::from_timestamp_millis(header.timestamp)
                        .expect("Expected NaiveDateTime")),
                    header: Set(serde_json::json!(header)),
                };
                let _ = block.insert(&tx).await;
                for ergotx in transactions.iter() {
                    current_transaction_id += 1;
                    let transaction = entities::transactions::Model {
                        transaction_id: ergotx.id.to_owned().unwrap(),
                        height: header.height,
                        id: current_transaction_id,
                    };
                    new_transactions.push(transaction.to_owned().into_active_model());
                    let mut input_index = 0;
                    for input in ergotx.inputs.iter() {
                        current_input_id += 1;
                        let utxo = match box_cached.cache_get(&input.box_id) {
                            Some(m) => m.to_owned(),
                            None => entities::boxes::Entity::find()
                                .filter(entities::boxes::Column::BoxId.eq(input.box_id.to_owned()))
                                .one(&tx)
                                .await
                                .unwrap()
                                .unwrap(),
                        };
                        let spent_txo = entities::boxes::Model {
                            spent: Some(header.height),
                            ..utxo
                        };
                        updated_boxes.push(spent_txo.id.to_owned());
                        box_cached.cache_set(input.box_id.to_owned(), spent_txo.to_owned());
                        let inp = entities::inputs::Model {
                            id: current_input_id,
                            box_id: box_cached.cache_get(&input.box_id).to_owned().unwrap().id,
                            transaction_id: transaction.id,
                            spending_proof: serde_json::json!(input.spending_proof),
                            index: input_index,
                        };
                        new_inputs.push(inp.into_active_model());
                        input_index += 1;
                    }
                    for outp in ergotx.outputs.iter() {
                        let address = match address_cached.cache_get(&outp.ergo_tree) {
                            Some(m) => m.to_owned(),
                            None => match entities::addresses::Entity::find()
                                .filter(entities::addresses::Column::Ergotree.eq(&outp.ergo_tree))
                                .one(&tx)
                                .await
                                .unwrap()
                            {
                                Some(m) => {
                                    address_cached.cache_set(m.ergotree.to_owned(), m.to_owned());
                                    m.to_owned()
                                }
                                None => {
                                    current_address_id += 1;
                                    let new_address = entities::addresses::Model {
                                        ergotree: outp.ergo_tree.to_owned(),
                                        ergotree_template: Some(hex::encode(
                                            &ErgoTree::sigma_parse_bytes(
                                                &hex::decode(outp.ergo_tree.to_owned()).unwrap(),
                                            )
                                            .unwrap()
                                            .template_bytes()
                                            .unwrap(),
                                        )),
                                        ergotree_hash: "".to_owned(),
                                        address: NetworkAddress::new(
                                            NetworkPrefix::Mainnet,
                                            &Address::recreate_from_ergo_tree(
                                                &ErgoTree::sigma_parse_bytes(
                                                    &hex::decode(outp.ergo_tree.to_owned())
                                                        .unwrap(),
                                                )
                                                .unwrap(),
                                            )
                                            .unwrap(),
                                        )
                                        .to_base58(),
                                        id: current_address_id,
                                    };
                                    new_addresses.push(new_address.to_owned().into_active_model());
                                    address_cached.cache_set(
                                        new_address.ergotree.to_owned(),
                                        new_address.to_owned(),
                                    );
                                    new_address
                                }
                            },
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
                        new_boxes.push(new_box.to_owned().into_active_model());
                        box_cached.cache_set(new_box.box_id.to_owned(), new_box.to_owned());
                        let mut asset_index = 0;
                        for ass in outp.assets.to_owned().unwrap().iter() {
                            current_token_in_box_id += 1;
                            let token = match token_cached.cache_get(&ass.token_id) {
                                Some(t) => t.to_owned(),
                                None => match entities::tokens::Entity::find()
                                    .filter(
                                        entities::tokens::Column::TokenId
                                            .eq(ass.token_id.to_owned()),
                                    )
                                    .one(&tx)
                                    .await
                                    .unwrap()
                                {
                                    Some(t) => {
                                        token_cached.cache_set(t.token_id.to_owned(), t.to_owned());
                                        t.to_owned()
                                    }
                                    None => {
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
                                                        .to_owned(),
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
                                                            .to_owned(),
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
                                        new_tokens.push(new_token.to_owned().into_active_model());
                                        token_cached.cache_set(
                                            new_token.token_id.to_owned(),
                                            new_token.to_owned(),
                                        );
                                        new_token
                                    }
                                },
                            };

                            let new_token_in_box = entities::token_in_box::Model {
                                box_id: new_box.id,
                                token_id: token.id,
                                amount: ass.amount,
                                index: asset_index,
                                id: current_token_in_box_id,
                            };

                            new_tokens_in_box.push(new_token_in_box.into_active_model());
                            asset_index += 1;
                        }
                    }
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

                if new_addresses.len() > 0 {
                    match entities::addresses::Entity::insert_many(
                        new_addresses.as_slice().to_owned(),
                    )
                    .exec(&tx)
                    .await
                    {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    };
                }
                match entities::transactions::Entity::insert_many(
                    new_transactions.as_slice().to_owned(),
                )
                .exec(&tx)
                .await
                {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                };
                for chunk in new_boxes.as_slice().chunks(1000) {
                    match entities::boxes::Entity::insert_many(chunk.to_owned())
                        .exec(&tx)
                        .await
                    {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    };
                }
                match entities::boxes::Entity::update_many()
                    .col_expr(
                        entities::boxes::Column::Spent,
                        Expr::value(Value::Int(Some(header.height))),
                    )
                    .filter(entities::boxes::Column::Id.is_in(updated_boxes.to_owned().into_iter()))
                    .exec(&tx)
                    .await
                {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                };
                if new_tokens.len() > 0 {
                    match entities::tokens::Entity::insert_many(new_tokens.as_slice().to_owned())
                        .exec(&tx)
                        .await
                    {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    };
                }

                if new_tokens_in_box.len() > 0 {
                    for chunk in new_tokens_in_box.as_slice().chunks(1000) {
                        match entities::token_in_box::Entity::insert_many(chunk.to_owned())
                            .exec(&tx)
                            .await
                        {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e),
                        };
                    }
                }
                match entities::inputs::Entity::insert_many(new_inputs.as_slice().to_owned())
                    .exec(&tx)
                    .await
                {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                };

                let _ = tx.commit().await;
                if work_block.zmq_mode {
                    info!(
                        "Inserted block with height: {} and header: {}",
                        header.height, header.id
                    );
                    let _ = socket.send(vec!["new_block", &header.id]).await;
                }
            }
        }
    }
}
