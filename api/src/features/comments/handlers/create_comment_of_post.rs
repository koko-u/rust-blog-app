use axum::extract;

use crate::errors;
use crate::features::comments::requests;
use crate::features::comments::responses;
use crate::features::posts::models as p_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreateCommentRequest,
    params(
        ("post_id" = p_models::PostId, Path, description = "Post Id")
    ),
    tag = "Blog Post Comments",
    responses(
        (status = 201, description = "Created new Category", body = responses::CommentResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_comment_of_post(
    extract::Path(post_id): extract::Path<p_models::PostId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateCommentRequest>,
) -> Result<shared::responses::Created<responses::CommentResponse>, errors::ApiError> {
    todo!()
}
