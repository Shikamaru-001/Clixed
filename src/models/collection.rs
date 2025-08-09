#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Collection {
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
    cover_image_id: Option<uuid::Uuid>,
}
