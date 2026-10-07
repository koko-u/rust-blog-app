use rslug::slugify;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostInput {
    pub title: String,
    pub user_id: uuid::Uuid,
    pub slug: String,
    pub content: Option<String>,
    pub category_name: String,
    pub tag_names: Vec<String>,
}

impl PostInput {
    pub fn new<Tags>(title: &str, user_id: uuid::Uuid, content: &str, category: &str, tags: Tags) -> Self
    where
        Tags: IntoIterator,
        Tags::Item: AsRef<str>,
    {
        Self {
            title: title.to_string(),
            user_id,
            slug: slugify!(title),
            content: Some(content.to_string()),
            category_name: category.to_string(),
            tag_names: tags.into_iter().map(|tag| tag.as_ref().to_string()).collect(),
        }
    }
}
