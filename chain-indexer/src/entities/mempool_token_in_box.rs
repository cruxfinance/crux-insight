//! `SeaORM` Entity for mempool_token_in_box

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mempool_token_in_box")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub mempool_box_id: i64,
    pub token_id: Option<i64>,
    pub mempool_token_id: Option<i64>,
    #[sea_orm(column_type = "Text")]
    pub token_id_str: String,
    pub amount: i64,
    pub index: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mempool_boxes::Entity",
        from = "Column::MempoolBoxId",
        to = "super::mempool_boxes::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MempoolBox,
    #[sea_orm(
        belongs_to = "super::tokens::Entity",
        from = "Column::TokenId",
        to = "super::tokens::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Token,
    #[sea_orm(
        belongs_to = "super::mempool_tokens::Entity",
        from = "Column::MempoolTokenId",
        to = "super::mempool_tokens::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MempoolToken,
}

impl Related<super::mempool_boxes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolBox.def()
    }
}

impl Related<super::tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl Related<super::mempool_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MempoolToken.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
