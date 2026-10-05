use axum::extract;

use crate::errors;
use crate::features::post_tags::repositories as pt_repositories;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::features::posts::requests;
use crate::features::posts::responses;
use crate::features::posts::services;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreatePostRequest,
    tag = "Blog Posts",
    responses(
        (status = 201, description = "Created new Blog Post", body = responses::PostResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_post(
    current_user: u_models::CurrentUser,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreatePostRequest>,
) -> Result<shared::responses::Created<responses::PostResponse>, errors::ApiError> {
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let command = request.validate_into(current_user.id, &state.pool).await?;

    let created: models::PostModel =
        shared::transaction::<models::PostModel, errors::ApiError, _>(&state.pool, async |tx| {
            // create post
            let post = services::tx_create_post(tx, &command).await?;

            // associate post and tags
            pt_repositories::insert_many_tag_ids(tx, current_user.id, post.id, command.tag_ids)
                .await
                .map_err(errors::ApiError::from)?;

            let posts = repositories::select_by_id(tx.conn(), post.id).await?;
            let mut posts: Vec<_> = posts.into();

            Ok(posts.pop().expect("should get created post"))
        })
        .await?;

    let location = format!("/api/posts/{}", created.id);
    Ok(shared::responses::created(location, created.into()))
}
