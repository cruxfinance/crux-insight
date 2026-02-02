//! `SeaORM` Entity for mempool_tokens

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mempool_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub token_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_description: Option<String>,
    pub decimals: Option<i32>,
    pub minted: Option<i64>,
    #[sea_orm(column_type = "Text")]
    pub issuer_box: String,
    pub issuance_box: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mempool_boxes::Entity",
        from = "Column::IssuanceBox",
        to = "super::mempool_boxes::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    IssuanceBox,
    #[sea_orm(has_many = "super::mempool_token_in_box::Entity")]
    MempoolTokenInBox,
}

impl Related<super::mempool_boxes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IssuanceBox.def()
    }
}

impl Related<super::mempool_token_in_box::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolTokenInBox.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
