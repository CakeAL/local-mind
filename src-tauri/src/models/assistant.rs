use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "assistant")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(indexed)]
    pub uuid: Uuid,
    pub name: String,     // 该助手名
    pub model: String,    // 调用模型名称 "deepseek-r1:8b"
    pub context_num: u64, // 上下文数, 默认: 5
    pub parameter: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantInfo {
    pub uuid: Uuid,
    pub name: String,
    pub model: String,
    pub context_num: u64,
}

impl From<Model> for AssistantInfo {
    fn from(m: Model) -> Self {
        Self {
            uuid: m.uuid,
            name: m.name,
            model: m.model,
            context_num: m.context_num,
        }
    }
}
