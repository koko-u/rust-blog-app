use crate::features::categories::models;
use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateCategoryCommand {
    pub id: models::CategoryId,
    pub user_id: shared::models::UserId,
    pub name: String,
    pub slug: String,
}
