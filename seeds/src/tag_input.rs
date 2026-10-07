#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagInput {
    pub name: String,
    pub user_id: uuid::Uuid,
}

impl TagInput {
    pub fn new(name: &str, user_id: uuid::Uuid) -> Self {
        Self {
            name: name.to_string(),
            user_id,
        }
    }
    pub fn to_name(&self) -> String {
        self.name.clone()
    }
    pub fn to_user_id(&self) -> uuid::Uuid {
        self.user_id
    }
}
