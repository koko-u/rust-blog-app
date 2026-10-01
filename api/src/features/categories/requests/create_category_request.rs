use crate::features::categories::commands;
use crate::shared::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreateCategoryRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,
}

impl CreateCategoryRequest {
    pub async fn validate_into(
        self,
        user_id: models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreateCategoryCommand, garde::Report> {
        use garde::Validate as _;
        let garde_result = self.validate();

        let mut name_result: Result<(), garde::Report> = Ok(());
        if let Some(name) = &self.name {
            // name のチェックについては未定
        }

        garde_result.map(|_| {
            let name = self.name.expect("category name should be required");
            let slug = rslug::slugify!(&name);

            commands::CreateCategoryCommand { user_id, name, slug }
        })
    }
}
