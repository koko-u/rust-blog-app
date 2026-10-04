use crate::features::categories::models as c_models;
use crate::features::posts::models;
use crate::features::tags::models as t_models;
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct PostRow {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub category_id: uuid::Uuid,
    pub category_user_id: uuid::Uuid,
    pub category_name: String,
    pub category_slug: String,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub tag_id: Option<uuid::Uuid>,
    pub tag_user_id: Option<uuid::Uuid>,
    pub tag_name: Option<String>,
}

impl From<PostRow> for models::PostModel {
    fn from(row: PostRow) -> Self {
        let tags = match (row.tag_id, row.tag_user_id, row.tag_name) {
            (Some(id), Some(user_id), Some(name)) => {
                vec![t_models::TagModel {
                    id: id.into(),
                    user_id: user_id.into(),
                    name,
                }]
            }
            _ => vec![],
        };
        Self {
            id: row.id.into(),
            user_id: row.user_id.into(),
            category: c_models::CategoryModel {
                id: row.category_id.into(),
                user_id: row.category_user_id.into(),
                name: row.category_name,
                slug: row.category_slug,
            },
            title: row.title,
            slug: row.slug,
            content: row.content,
            tags,
        }
    }
}
