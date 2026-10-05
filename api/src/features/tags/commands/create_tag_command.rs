use crate::features::users::models as u_models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateTagCommand {
    pub user_id: u_models::UserId,
    pub name: String,
}
