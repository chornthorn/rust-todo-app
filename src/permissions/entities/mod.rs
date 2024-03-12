use sqlx::FromRow;

#[derive(FromRow)]
pub struct Permission {
    id: u32,
    name: Option<String>,
    module: String,
    action: String,
    order: usize
}