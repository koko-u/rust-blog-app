use axum::extract;

use crate::errors;
use crate::features::posts::repositories;
use crate::features::posts::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    description = "Get All Blog Posts",
    tag = "Blog Posts",
    responses(
        (status = 200, description = "List of Categories", body = Vec<responses::PostResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_posts(
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::PostResponse>>, errors::ApiError> {
    let posts = repositories::select_all(&state.pool).await?;
    let posts: Vec<_> = posts.into();
    let response = posts.into_iter().map(responses::PostResponse::from).collect();

    Ok(axum::Json(response))
}
