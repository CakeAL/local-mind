use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "knowledge_base")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(indexed, unique)]
    pub name: String, // 知识库名称
    pub model: String,         // 嵌入模型名称 "bge-m3:567m"
    pub request_text_num: i32, // 请求文档片段数量, 默认: 6
    pub segmenting_size: i32,  // 将文档分段大小，默认：1024
    pub match_threshold: f64,  // 匹配度阈值，默认: 0.5
    pub file_paths: Json,      // Vec<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct KnowledgeBaseInfo {
    pub id: i64,
    pub name: String,
    pub model: String,
}

impl From<Model> for KnowledgeBaseInfo {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            model: m.model,
        }
    }
}
