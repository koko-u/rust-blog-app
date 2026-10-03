use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateTagCommand {
    pub user_id: shared::models::UserId,
    pub name: String,
}
