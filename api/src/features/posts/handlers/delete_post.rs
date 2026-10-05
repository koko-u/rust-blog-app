use axum::extract;
use axum::http;

use crate::errors;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::features::users::models as u_models;
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
    current_user: u_models::CurrentUser,
    extract::Path(id): extract::Path<models::PostId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    shared::transaction(&state.pool, async |tx| {
        let affected_rows = repositories::delete_by_id_user_id(tx, id, current_user.id).await?;
        (affected_rows > 0).ok_or_else(|| errors::ApiError::NotFound {
            message: format!("Post with Id={id} not found"),
        })
    })
    .await?;

    Ok(http::StatusCode::NO_CONTENT)
}
