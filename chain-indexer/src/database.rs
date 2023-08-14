use sea_orm::{Database, DatabaseConnection};
use std::env;

use crate::settings::Settings;

pub struct CIDatabase {
    pub settings: Settings,
}

impl CIDatabase {
    pub async fn connect(&self) -> DatabaseConnection {
        Database::connect(format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").unwrap_or(self.settings.database.user.to_owned()),
            env::var("POSTGRES_PASSWORD").unwrap_or(self.settings.database.password.to_owned()),
            env::var("POSTGRES_HOST").unwrap_or(self.settings.database.host.to_owned()),
            env::var("POSTGRES_PORT").unwrap_or(self.settings.database.port.to_owned()),
            env::var("POSTGRES_DB").unwrap_or(self.settings.database.db.to_owned()),
        ))
        .await
        .expect("Failed creating db conn")
    }
}
