#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ImageCollection {
    id: uuid::Uuid,
    image_id: Option<uuid::Uuid>,
    collection_id: Option<uuid::Uuid>,
}
