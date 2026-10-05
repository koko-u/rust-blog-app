use crate::features::tags::models;
use crate::features::users::models as u_models;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateTagCommand {
    pub id: models::TagId,
    pub user_id: u_models::UserId,
    pub name: String,
}
