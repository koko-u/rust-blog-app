use axum::extract;

use crate::errors;
use crate::features::posts::repositories;
use crate::features::posts::responses;
use crate::features::users::models as u_models;
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
    current_user: u_models::CurrentUser,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::PostResponse>>, errors::ApiError> {
    let posts = repositories::select_by_user_id(&state.pool, current_user.id).await?;
    let posts: Vec<_> = posts.into();
    let response = posts.into_iter().map(responses::PostResponse::from).collect();

    Ok(axum::Json(response))
}
