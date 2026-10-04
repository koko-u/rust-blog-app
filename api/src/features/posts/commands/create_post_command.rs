use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreatePostCommand {
    pub user_id: shared::models::UserId,
    pub category_name: String,
    pub title: String,
    pub content: Option<String>,
    pub tag_names: Vec<String>,
}
