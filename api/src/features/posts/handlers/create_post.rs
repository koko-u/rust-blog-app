use axum::extract;

use crate::errors;
use crate::features::categories::commands as c_commands;
use crate::features::categories::repositories as c_repositories;
use crate::features::categories::services as c_services;
use crate::features::post_tags::repositories as pt_repositories;
use crate::features::posts::models;
use crate::features::posts::requests;
use crate::features::posts::responses;
use crate::features::posts::services;
use crate::features::tags::models as t_models;
use crate::features::tags::repositories as t_repositories;
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
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreatePostRequest>,
) -> Result<shared::responses::Created<responses::PostResponse>, errors::ApiError> {
    // TODO get user_id from authentication
    let user_id = uuid::uuid!("c85df66a-ccd7-4f23-9df6-6accd7ff23db").into();
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let command = request.validate_into(user_id, &state.pool).await?;

    let created: models::PostModel =
        shared::transaction::<models::PostModel, errors::ApiError, _>(&state.pool, async |tx| {
            // create or get category
            let categories =
                c_repositories::select_by_user_id_name(&state.pool, user_id, &command.category_name)
                    .await
                    .map_err(errors::ApiError::from)?;
            let category_id = match categories.first() {
                Some(c) => c.id.into(),
                None => {
                    let cmd = c_commands::CreateCategoryCommand {
                        user_id,
                        name: command.category_name.clone(),
                    };
                    let category = c_services::tx_create_category(&cmd, tx).await?;
                    category.id
                }
            };

            // create tags
            let mut tags = Vec::<t_models::TagModel>::new();
            for tag_name in command.tag_names.iter() {
                let tag = t_repositories::insert_or_select(tx, user_id, tag_name)
                    .await
                    .map_err(errors::ApiError::from)?;
                tags.push(tag.into());
            }

            // create post
            let mut post = services::tx_create_post(tx, &command, category_id).await?;

            // associate post and tags
            pt_repositories::insert_many_tag_ids(tx, user_id, post.id, tags.iter().map(|t| t.id))
                .await
                .map_err(errors::ApiError::from)?;

            post.tags.extend(tags);
            Ok(post)
        })
        .await?;

    let location = format!("/api/posts/{}", created.id);
    Ok(shared::responses::created(location, created.into()))
}
