use axum::extract;
use axum::http;

use crate::errors;
use crate::features::posts::models;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = models::PostId, Path, description = "Blog Post Id")
    ),
    description = "Delete Blog Post by Id",
    tag = "Blog Posts",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "the Product not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn delete_post(
    extract::Path(id): extract::Path<models::PostId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    todo!()
}
