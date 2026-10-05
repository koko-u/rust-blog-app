use axum::extract;

use crate::errors;
use crate::features::categories::requests;
use crate::features::categories::responses;
use crate::features::categories::services;
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
    let user_id = uuid::uuid!("01a106a7-4327-7287-9940-af4254498604").into();
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let command = request.validate_into(user_id, &state.pool).await?;

    let created = shared::transaction(&state.pool, async |tx| {
        services::tx_create_category(&command, tx).await
    })
    .await?;

    let location = format!("/api/categories/{}", created.id);
    Ok(shared::responses::created(location, created.into()))
}
