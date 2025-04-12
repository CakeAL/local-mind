use uuid::Uuid;
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "assistant")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    #[sea_orm(indexed)]
    uuid: Uuid,
    name: String,
    parameter: Json
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
