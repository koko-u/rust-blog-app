use axum::extract;

use crate::errors;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::tags::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    description = "Get All Tags",
    tag = "Tags",
    responses(
        (status = 200, description = "List of Categories", body = Vec<responses::TagResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_tags(
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::TagResponse>>, errors::ApiError> {
    let tags = repositories::select_all(&state.pool).await?;

    let response = tags
        .into_iter()
        .map(models::TagModel::from)
        .map(responses::TagResponse::from)
        .collect();

    Ok(axum::Json(response))
}
