use axum::extract;

use crate::errors;
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
    todo!()
}
