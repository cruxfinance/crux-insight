//! `SeaORM` Entity for mempool_boxes

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mempool_boxes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub box_id: String,
    pub address: Option<i64>,
    #[sea_orm(column_type = "Text")]
    pub ergotree: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,
    pub mempool_transaction_id: i64,
    pub output_index: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mempool_transactions::Entity",
        from = "Column::MempoolTransactionId",
        to = "super::mempool_transactions::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MempoolTransaction,
    #[sea_orm(has_many = "super::mempool_token_in_box::Entity")]
    MempoolTokenInBox,
    #[sea_orm(has_many = "super::mempool_tokens::Entity")]
    MempoolTokensIssuer,
}

impl Related<super::mempool_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolTransaction.def()
    }
}

impl Related<super::mempool_token_in_box::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolTokenInBox.def()
    }
}

impl Related<super::mempool_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolTokensIssuer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
