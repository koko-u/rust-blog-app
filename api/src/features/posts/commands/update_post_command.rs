use crate::features::categories::models as c_models;
use crate::features::posts::models;
use crate::features::tags::models as t_models;
use crate::features::users::models as u_models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdatePostCommand {
    pub id: models::PostId,
    pub user_id: u_models::UserId,
    pub category_id: c_models::CategoryId,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub tag_ids: Vec<t_models::TagId>,
}
