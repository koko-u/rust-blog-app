use axum::extract;

use crate::errors;
use crate::features::tags::commands;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::tags::requests;
use crate::features::tags::responses;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreateTagRequest,
    tag = "Tags",
    responses(
        (status = 201, description = "Created new Category", body = responses::TagResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_tag(
    current_user: u_models::CurrentUser,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateTagRequest>,
) -> Result<shared::responses::Created<responses::TagResponse>, errors::ApiError> {
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let commands::CreateTagCommand { user_id, name } =
        request.validate_into(current_user.id, &state.pool).await?;

    let created: models::TagModel = shared::transaction(&state.pool, async |tx| {
        let row = repositories::insert_optional(tx, user_id, &name).await?;
        match row {
            Some(row) => Ok(models::TagModel::from(row)),
            None => Err(errors::ApiError::Other {
                message: "Failed to create tag data, it may duplicate the name".to_string(),
            }),
        }
    })
    .await?;

    let location = format!("/api/tags/{}", created.id);
    Ok(shared::responses::created(location, created.into()))
}
