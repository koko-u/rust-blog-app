use axum::extract;

use crate::errors;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::features::posts::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = models::PostId, Path, description = "Blog Post Id")
    ),
    description = "Get Single Blog Post by Id",
    tag = "Blog Posts",
    responses(
        (status = 200, description = "Category of id", body = responses::PostResponse),
        (status = 404, description = "the Category is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_post(
    extract::Path(id): extract::Path<models::PostId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::PostResponse>, errors::ApiError> {
    let posts = repositories::select_by_id(&state.pool, id).await?;
    let posts: Vec<_> = posts.into();

    match posts.into_iter().next() {
        Some(post) => {
            let post: responses::PostResponse = post.into();
            Ok(axum::Json(post))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Post with Id={id} not found"),
        }),
    }
}
