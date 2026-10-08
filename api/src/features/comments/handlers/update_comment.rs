use axum::extract;

use crate::errors;
use crate::features::comments::commands;
use crate::features::comments::models;
use crate::features::comments::path_params;
use crate::features::comments::repositories;
use crate::features::comments::requests;
use crate::features::comments::responses;
use crate::features::users::models as u_models;
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
    current_user: u_models::CurrentUser,
    extract::Path(params): extract::Path<path_params::PostCommentParam>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateCommentRequest>,
) -> Result<axum::Json<responses::CommentResponse>, errors::ApiError> {
    let path_params::PostCommentParam { post_id, comment_id } = params;
    // validate request
    let commands::UpdateCommentCommand {
        id,
        post_id,
        user_id,
        content,
    } = request
        .validate_into(comment_id, post_id, current_user.id, &state.pool)
        .await?;

    let updated = shared::transaction::<models::CommentModel, sqlx::Error, _>(&state.pool, async |tx| {
        // update category
        let row = repositories::update(tx, id, post_id, user_id, &content).await?;

        Ok(models::CommentModel::from(row))
    })
    .await?;

    Ok(axum::Json(updated.into()))
}
