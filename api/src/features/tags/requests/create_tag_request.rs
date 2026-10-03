use crate::errors;
use crate::features::tags::commands;
use crate::features::tags::repositories;
use crate::shared;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreateTagRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,
}

impl CreateTagRequest {
    pub async fn validate_into(
        self,
        user_id: shared::models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreateTagCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // check user_id and name are unique
        let mut name_result = Ok(());
        if let Some(name) = &self.name {
            let exists = repositories::exists_by_user_id_name(&pool, user_id, name).await?;
            if exists {
                let error = garde::Error::new("The user_id and name are already registered.");
                let mut report = garde::Report::new();
                report.append(garde::Path::new("user_id"), error.clone());
                report.append(garde::Path::new("name"), error.clone());
                name_result = Err(report);
            }
        }

        match (garde_result, name_result) {
            (Ok(()), Ok(())) => Ok(commands::CreateTagCommand {
                user_id,
                name: self.name.expect("name should be required"),
            }),
            (r1, r2) => {
                let report = merge!(r1, r2);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
