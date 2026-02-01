//! `SeaORM` Entity for mempool_transactions

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mempool_transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub transaction_id: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,
    pub first_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::mempool_boxes::Entity")]
    MempoolBoxes,
    #[sea_orm(has_many = "super::mempool_inputs::Entity")]
    MempoolInputs,
}

impl Related<super::mempool_boxes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolBoxes.def()
    }
}

impl Related<super::mempool_inputs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolInputs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
