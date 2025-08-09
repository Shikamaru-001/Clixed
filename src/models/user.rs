#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
 pub    id: uuid::Uuid,
    pub username: String,
}
