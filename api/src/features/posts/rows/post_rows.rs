use super::post_row;
use crate::features::categories::models as c_models;
use crate::features::posts::models;
use crate::features::tags::models as t_models;

#[derive(Debug, Clone, Eq, PartialEq, derive_more::From, derive_more::Deref, derive_more::DerefMut)]
pub struct PostRows(Vec<post_row::PostRow>);

impl PostRows {
    pub fn into_inner(self) -> Vec<post_row::PostRow> {
        self.0
    }
}

impl From<PostRows> for Vec<models::PostModel> {
    fn from(rows: PostRows) -> Self {
        use itertools::Itertools as _;

        let mut posts = vec![];
        // group by post key
        for (key, chunk) in rows
            .iter()
            .sorted_by_key(|row| row.id)
            .chunk_by(|&row| Key::from(row.clone()))
            .into_iter()
        {
            let mut post: models::PostModel = key.into();
            // collect tag info
            post.tags = chunk
                .flat_map(|row| match (row.tag_id, row.tag_user_id, &row.tag_name) {
                    (Some(id), Some(user_id), Some(name)) => Some(t_models::TagModel {
                        id: id.into(),
                        user_id: user_id.into(),
                        name: name.to_string(),
                    }),
                    _ => None,
                })
                .collect();

            posts.push(post);
        }

        posts
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Key {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    category: c_models::CategoryModel,
    title: String,
    slug: String,
    content: Option<String>,
}
impl From<post_row::PostRow> for Key {
    fn from(row: post_row::PostRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            category: c_models::CategoryModel {
                id: row.category_id.into(),
                user_id: row.category_user_id.into(),
                name: row.category_name,
                slug: row.category_slug,
            },
            title: row.title,
            slug: row.slug,
            content: row.content,
        }
    }
}
impl From<Key> for models::PostModel {
    fn from(key: Key) -> Self {
        Self {
            id: key.id.into(),
            user_id: key.user_id.into(),
            category: key.category,
            title: key.title,
            slug: key.slug,
            content: key.content,
            tags: vec![],
        }
    }
}
