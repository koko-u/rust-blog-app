use crate::errors;
use crate::features::tags::commands;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::users::models as u_models;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdateTagRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,
}

impl UpdateTagRequest {
    pub async fn validate_into(
        self,
        id: models::TagId,
        user_id: u_models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::UpdateTagCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // check exists id and user_id
        let exists = repositories::exists_by_id_user_id(pool, id, user_id).await?;
        if !exists {
            return Err(errors::ValidationError::NoResource {
                message: "There is no tags corresponding to id and user_id".to_string(),
            });
        }

        // user_id and name should not be duplicate
        let mut name_result = Ok(());
        if let Some(name) = &self.name {
            let row = repositories::select_by_user_id_name(pool, user_id, name).await?;
            if let Some(row) = row
                && row.id != id.into_inner()
            {
                // find name which id is not self id
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("name"),
                    garde::Error::new(format!("The name to be changed({name}) is duplicated.")),
                );
                name_result = Err(report);
            }
        }

        match (garde_result, name_result) {
            (Ok(()), Ok(())) => Ok(commands::UpdateTagCommand {
                id,
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
