use sea_orm::{entity::prelude::*, FromQueryResult};
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
    pub context_num: i64, // 上下文数, 默认: 5
    pub parameter: Json,
    pub knowledge_base: Option<String>, // 连接的数据库
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct AssistantInfo {
    pub id: i64,
    pub uuid: Uuid,
    pub name: String,
    pub model: String,
    pub context_num: i64,
    pub knowledge_base: Option<String>,
}

impl From<Model> for AssistantInfo {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            uuid: m.uuid,
            name: m.name,
            model: m.model,
            context_num: m.context_num,
            knowledge_base: m.knowledge_base,
        }
    }
}
