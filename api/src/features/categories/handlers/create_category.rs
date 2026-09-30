use axum::extract;

use crate::errors;
use crate::features::categories::commands;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::features::categories::requests;
use crate::features::categories::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreateCategoryRequest,
    tag = "Categories",
    responses(
        (status = 201, description = "Created new Category", body = responses::CategoryResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_category(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateCategoryRequest>,
) -> Result<shared::responses::Created<responses::CategoryResponse>, errors::ApiError> {
    // TODO get user_id from authentication
    let user_id = uuid::uuid!("c85df66a-ccd7-4f23-9df6-6accd7ff23db").into();
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let mut command = request.validate_into(user_id, &state.pool).await?;

    let created: models::CategoryModel = shared::transaction(&state.pool, async |tx| {
        // try to create category
        let base_slug = command.slug.clone();
        let mut i = 1;
        let created_row = loop {
            if i >= 100 {
                return Err(errors::ApiError::Other {
                    message: "The number of attempts to generate the slug has been exceeded.".to_string(),
                });
            }
            let row = repositories::insert_optional(tx, &command).await?;
            if let Some(row) = row {
                break models::CategoryModel::from(row);
            }

            let slug = format!("{base_slug}-{i}");
            command = commands::CreateCategoryCommand { slug, ..command };

            i += 1;
        };

        Ok(created_row)
    })
    .await?;

    let location = format!("/api/categories/{}", created.id);
    Ok(shared::responses::created(location, created.into()))
}
