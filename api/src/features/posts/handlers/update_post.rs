use axum::extract;

use crate::errors;
use crate::features::posts::models;
use crate::features::posts::requests;
use crate::features::posts::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::PostId, Path, description = "Blog Post Id")
    ),
    request_body = requests::UpdatePostRequest,
    tag = "Blog Posts",
    responses(
        (status = 200, description = "the Updated Category", body = responses::PostResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_post(
    extract::Path(id): extract::Path<models::PostId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdatePostRequest>,
) -> Result<axum::Json<responses::PostResponse>, errors::ApiError> {
    todo!()
}
