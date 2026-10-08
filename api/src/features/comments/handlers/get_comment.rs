use axum::extract;

use crate::errors;
use crate::features::comments::models;
use crate::features::comments::path_params;
use crate::features::comments::repositories;
use crate::features::comments::responses;
use crate::features::users::models as u_models;
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
    current_user: u_models::CurrentUser,
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::CommentResponse>, errors::ApiError> {
    let path_params::PostCommentParam { post_id, comment_id } = params;
    let comment =
        repositories::select_by_id_post_id_user_id(&state.pool, comment_id, post_id, current_user.id).await?;

    match comment {
        Some(comment) => {
            let comment: models::CommentModel = comment.into();
            let comment: responses::CommentResponse = comment.into();
            Ok(axum::Json(comment))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Comment with Id={comment_id} not found"),
        }),
    }
}
