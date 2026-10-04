use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateCategoryCommand {
    pub user_id: shared::models::UserId,
    pub name: String,
}
