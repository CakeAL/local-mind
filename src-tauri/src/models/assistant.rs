use uuid::Uuid;
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "assistant")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    #[sea_orm(indexed)]
    uuid: Uuid,
    name: String, // 该助手名
    model: String, // 调用模型名称 "deepseek-r1:8b"
    context_num: u64, // 上下文数, 默认: 5
    parameter: Json
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
