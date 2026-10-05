use crate::features::categories::models;
use crate::features::users::models as u_models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateCategoryCommand {
    pub id: models::CategoryId,
    pub user_id: u_models::UserId,
    pub name: String,
    pub slug: String,
}
