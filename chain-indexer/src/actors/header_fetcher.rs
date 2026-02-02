use std::{
    cmp::min,
    time::{Duration, Instant},
};

use ergo_node_client::apis::{blocks_api, configuration::Configuration};
use futures::StreamExt;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tmq::{subscribe, Context};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use tracing::{debug, error, info};

use crate::database::CIDatabase;
use crate::entities::blocks;
use crate::settings::Settings;
use crate::types::work_object::WorkBlock;

async fn get_header_id(settings: &Settings, height: i32) -> String {
    info!("Looking for header with height: {}", height);
    let db = CIDatabase {
        settings: settings.to_owned(),
    }
    .connect()
    .await;
    let res = blocks::Entity::find()
        .filter(blocks::Column::Height.eq(height))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .header_id;
    let _ = db.close().await;
    res
}

pub async fn fetch_headers(
    node_conf: &Configuration,
    from: i32,
    to: i32,
    sender: Sender<Vec<WorkBlock>>,
) {
    let settings = Settings::new().unwrap();
    let mut last_header_id = get_header_id(&settings, from).await;
    let mut offset = from;
    let limit = 10;
    while offset < to {
        let fetch_start = Instant::now();
        let headers_res =
            blocks_api::get_chain_slice(&node_conf, Some(offset), Some(min(offset + limit, to)))
                .await;
        match headers_res {
            Ok(headers) => {
                if offset % 100 == 0 {
                    debug!(
                        "fetch_headers at {} took {:?}",
                        offset,
                        fetch_start.elapsed()
                    );
                }
                offset += headers.len() as i32;
                if headers.first().unwrap().parent_id == last_header_id {
                    last_header_id = headers.last().unwrap().id.clone();
                    match sender
                        .send(
                            headers
                                .iter()
                                .map(|header| WorkBlock {
                                    zmq_mode: false,
                                    header: Some(header.clone()),
                                    transactions: None,
                                    rollback_height: None,
                                })
                                .collect(),
                        )
                        .await
                    {
                        Ok(_res) => continue,
                        Err(error) => panic!("{}", error),
                    }
                } else {
                    offset = headers.first().unwrap().height - 10;
                    last_header_id = get_header_id(&settings, offset).await;
                    match sender
                        .send(vec![WorkBlock {
                            zmq_mode: false,
                            header: None,
                            transactions: None,
                            rollback_height: Some(offset),
                        }])
                        .await
                    {
                        Ok(_res) => (),
                        Err(error) => panic!("{}", error),
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
                sleep(Duration::from_millis(1000_u64)).await;
            }
        }
    }
    info!("Done fetching headers, switching to zmq mode");
    let mut socket = subscribe(&Context::new())
        .connect(&settings.ergo_node.zmq_url)
        .unwrap()
        .subscribe(b"newBlock")
        .unwrap();

    loop {
        let msg_mp = socket.next().await.unwrap().unwrap();
        if sender.is_closed() {
            panic!("No more receivers.")
        }
        let header_id = msg_mp
            .iter()
            .map(|item| item.as_str().unwrap_or("invalid text"))
            .collect::<Vec<&str>>()[1];
        debug!("{}", header_id);

        let mut success = false;
        while !success {
            let headers_res =
                blocks_api::get_chain_slice(&node_conf, Some(offset), Some(offset + limit)).await;
            match headers_res {
                Ok(headers) => {
                    offset += headers.len() as i32;
                    for header in headers.iter() {
                        if header.parent_id == last_header_id {
                            last_header_id = header.id.clone();
                            match sender
                                .send(vec![WorkBlock {
                                    zmq_mode: true,
                                    header: Some(header.clone()),
                                    transactions: None,
                                    rollback_height: None,
                                }])
                                .await
                            {
                                Ok(_) => (),
                                Err(e) => panic!("{}", e),
                            }

                            success = true;
                        } else {
                            offset = header.height - 10;
                            last_header_id = get_header_id(&settings, offset).await;
                            match sender
                                .send(vec![WorkBlock {
                                    zmq_mode: true,
                                    header: None,
                                    transactions: None,
                                    rollback_height: Some(offset),
                                }])
                                .await
                            {
                                Ok(_res) => break,
                                Err(error) => panic!("{}", error),
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    tokio::time::sleep(Duration::new(1, 0)).await;
                }
            }
        }
    }
}
