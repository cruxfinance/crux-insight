use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ErgoNode {
    pub url: String,
    pub zmq_url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ChainIndexer {
    pub tx_par: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Crux {
    pub pubsubport: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mempool {
    pub enabled: bool,
    pub full_resync_interval: i32,
}

impl Default for Mempool {
    fn default() -> Self {
        Mempool {
            enabled: true,
            full_resync_interval: 10,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub database: Database,
    pub ergo_node: ErgoNode,
    pub chain_indexer: ChainIndexer,
    pub crux: Crux,
    #[serde(default)]
    pub mempool: Mempool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("./config/default"))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("local").required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
