use axum::extract;
use axum::http;

use crate::errors;
use crate::features::comments::path_params;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/{comment_id}",
    params(
        path_params::PostCommentParam
    ),
    description = "Delete Comment by Id",
    tag = "Blog Post Comments",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "the Product not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn delete_comment(
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    todo!()
}
