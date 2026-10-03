use crate::features::tags::models;
use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateTagCommand {
    pub id: models::TagId,
    pub user_id: shared::models::UserId,
    pub name: String,
}
