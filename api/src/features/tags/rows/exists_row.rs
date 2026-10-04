#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ExistsRow {
    pub name: String,
    pub exists: bool,
}
