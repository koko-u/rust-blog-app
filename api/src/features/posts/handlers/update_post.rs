use axum::extract;

use crate::errors;
use crate::features::post_tags::repositories as pt_repositories;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::features::posts::requests;
use crate::features::posts::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::PostId, Path, description = "Blog Post Id")
    ),
    request_body = requests::UpdatePostRequest,
    tag = "Blog Posts",
    responses(
        (status = 200, description = "the Updated Category", body = responses::PostResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_post(
    extract::Path(id): extract::Path<models::PostId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdatePostRequest>,
) -> Result<axum::Json<responses::PostResponse>, errors::ApiError> {
    // TODO get user_id from authentication
    let user_id = uuid::uuid!("01a106a7-4327-7287-9940-af4254498604").into();
    // validate request
    let command = request.validate_into(id, user_id, &state.pool).await?;

    let posts = shared::transaction::<Vec<_>, sqlx::Error, _>(&state.pool, async |tx| {
        // update post
        repositories::update(tx, &command).await?;
        // de-associate tags
        pt_repositories::delete_tags_of_post(tx, command.user_id, command.id).await?;
        // associate tags
        pt_repositories::insert_many_tag_ids(tx, command.user_id, command.id, command.tag_ids).await?;

        // get updated post
        let posts = repositories::select_by_id(tx.conn(), command.id).await?;
        Ok(posts.into())
    })
    .await?;

    let response = match posts.into_iter().next() {
        Some(post) => post.into(),
        None => {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::empty(),
                garde::Error::new("Cannot update the post, it may conflict the row"),
            );
            return Err(errors::ApiError::Validation(report));
        }
    };

    Ok(axum::Json(response))
}
