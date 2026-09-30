#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateCategoryCommand {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
}
