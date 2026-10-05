#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CategoryInput {
    pub name: String,
    pub slug: String,
}
impl CategoryInput {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            slug: rslug::slugify!(name),
        }
    }

    pub fn to_name(&self) -> String {
        self.name.clone()
    }

    pub fn to_slug(&self) -> String {
        self.slug.clone()
    }
}
