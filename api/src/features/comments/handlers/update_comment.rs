use axum::extract;

use crate::errors;
use crate::features::comments::path_params;
use crate::features::comments::requests;
use crate::features::comments::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{comment_id}",
    params(
        path_params::PostCommentParam
    ),
    request_body = requests::UpdateCommentRequest,
    tag = "Blog Post Comments",
    responses(
        (status = 200, description = "the Updated Category", body = responses::CommentResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_comment(
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateCommentRequest>,
) -> Result<axum::Json<responses::CommentResponse>, errors::ApiError> {
    todo!()
}
