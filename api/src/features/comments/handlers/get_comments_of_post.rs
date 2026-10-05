use axum::extract;

use crate::errors;
use crate::features::comments::responses;
use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    params(
        ("post_id" = p_models::PostId, Path, description = "Post Id")
    ),
    description = "Get All Comments of the Post",
    tag = "Blog Post Comments",
    responses(
        (status = 200, description = "List of Categories", body = Vec<responses::CommentResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_comments_of_post(
    current_user: u_models::CurrentUser,
    extract::Path(post_id): extract::Path<p_models::PostId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::CommentResponse>>, errors::ApiError> {
    todo!()
}
