#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Image {
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
}
