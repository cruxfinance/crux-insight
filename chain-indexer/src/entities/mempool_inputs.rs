//! `SeaORM` Entity for mempool_inputs

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mempool_inputs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub mempool_transaction_id: i64,
    #[sea_orm(column_type = "Text")]
    pub box_id: String,
    pub confirmed_box_id: Option<i64>,
    #[sea_orm(column_type = "JsonBinary")]
    pub spending_proof: Json,
    pub index: i32,
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
    #[sea_orm(
        belongs_to = "super::boxes::Entity",
        from = "Column::ConfirmedBoxId",
        to = "super::boxes::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ConfirmedBox,
}

impl Related<super::mempool_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolTransaction.def()
    }
}

impl Related<super::boxes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConfirmedBox.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
