//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "boxes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub address: i64,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,
    pub created: i32,
    pub spent: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub box_id: String,
    pub transaction_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Created",
        to = "super::blocks::Column::Height",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Blocks2,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Spent",
        to = "super::blocks::Column::Height",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Blocks1,
}

impl ActiveModelBehavior for ActiveModel {}
