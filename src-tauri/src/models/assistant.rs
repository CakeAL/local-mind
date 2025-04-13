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
    uuid: Uuid,
    name: String,
    model: String,
    context_num: u64,
}

impl Into<AssistantInfo> for Model {
    fn into(self) -> AssistantInfo {
        AssistantInfo {
            uuid: self.uuid,
            name: self.name,
            model: self.model,
            context_num: self.context_num,
        }
    }
}
