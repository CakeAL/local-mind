use ollama_rust_api::model::chat::MessageRole;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(indexed)]
    pub assistant_uuid: Uuid,
    pub model: String,
    pub created_at: DateTimeUtc,
    pub content: String,
    pub role: RoleType,
    pub total_duration: i64, // 纳秒
    pub load_duration: i64,
    pub prompt_eval_count: i64,
    pub prompt_eval_duration: i64,
    pub eval_count: i64,
    pub eval_duration: i64,
    pub search_result: Option<Json>, // 存储向量相似度搜索结果 assistant 对话
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Eq, Clone, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum RoleType {
    #[sea_orm(string_value = "S")]
    System,
    #[sea_orm(string_value = "U")]
    User,
    #[sea_orm(string_value = "A")]
    Assistant,
    #[sea_orm(string_value = "T")]
    Tool,
}

impl From<RoleType> for MessageRole {
    fn from(rt: RoleType) -> Self {
        match rt {
            RoleType::User => MessageRole::User,
            RoleType::System => MessageRole::System,
            RoleType::Assistant => MessageRole::Assistant,
            RoleType::Tool => MessageRole::Tool,
        }
    }
}
