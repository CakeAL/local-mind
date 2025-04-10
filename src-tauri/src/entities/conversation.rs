use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub assistant_uuid: Uuid,
    pub model: String,
    pub created_at: DateTimeUtc,
    pub content: String,
    pub role: String,
    pub total_duration: i64, // 纳秒
    pub load_duration: i64,
    pub prompt_eval_count: i64,
    pub prompt_eval_duration: i64,
    pub eval_count: i64,
    pub eval_druation: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assistant::Entity",
        from = "Column::AssistantUuid",
        to = "super::assistant::Column::Uuid",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Assistant,
}

impl Related<super::assistant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assistant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
