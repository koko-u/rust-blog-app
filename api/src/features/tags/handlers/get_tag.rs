use axum::extract;

use crate::errors;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::tags::responses;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = models::TagId, Path, description = "Tag Id")
    ),
    description = "Get Single Tag by Id",
    tag = "Tags",
    responses(
        (status = 200, description = "Category of id", body = responses::TagResponse),
        (status = 404, description = "the Category is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_tag(
    current_user: u_models::CurrentUser,
    extract::Path(id): extract::Path<models::TagId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::TagResponse>, errors::ApiError> {
    let tag = repositories::select_by_id_user_id(&state.pool, id, current_user.id).await?;

    match tag {
        Some(tag) => {
            let tag: models::TagModel = tag.into();
            let tag: responses::TagResponse = tag.into();
            Ok(axum::Json(tag))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Tag with Id={id} not found"),
        }),
    }
}
