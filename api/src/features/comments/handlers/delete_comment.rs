use axum::extract;
use axum::http;

use crate::errors;
use crate::features::comments::models;
use crate::features::comments::path_params;
use crate::features::comments::repositories;
use crate::features::users::models as u_models;
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
    current_user: u_models::CurrentUser,
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    let path_params::PostCommentParam { post_id, comment_id } = params;

    shared::transaction(&state.pool, async |tx| {
        let row =
            repositories::delete_by_id_post_id_user_id(tx, comment_id, post_id, current_user.id).await?;

        match row {
            Some(row) => Ok(models::CommentModel::from(row)),
            None => Err(errors::ApiError::NotFound {
                message: format!("Comment with Id={comment_id} not found"),
            }),
        }
    })
    .await?;

    Ok(http::StatusCode::NO_CONTENT)
}
