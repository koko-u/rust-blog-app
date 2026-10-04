use axum::extract;

use crate::errors;
use crate::features::comments::path_params;
use crate::features::comments::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{comment_id}",
    params(
        path_params::PostCommentParam
    ),
    description = "Get Comment by Id",
    tag = "Blog Post Comments",
    responses(
        (status = 200, description = "Category of id", body = responses::CommentResponse),
        (status = 404, description = "the Category is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_comment(
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::CommentResponse>, errors::ApiError> {
    todo!()
}
