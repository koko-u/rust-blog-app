use axum::extract;
use axum::http;

use crate::errors;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = models::TagId, Path, description = "Tag Id")
    ),
    description = "Delete Tag by Id",
    tag = "Tags",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "the Product not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn delete_tag(
    current_user: u_models::CurrentUser,
    extract::Path(id): extract::Path<models::TagId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    shared::transaction(&state.pool, async |tx| {
        let row = repositories::delete_by_id_user_id(tx, id, current_user.id).await?;

        match row {
            Some(row) => Ok(models::TagModel::from(row)),
            None => Err(errors::ApiError::NotFound {
                message: format!("Tag with Id={id} not found"),
            }),
        }
    })
    .await?;

    Ok(http::StatusCode::NO_CONTENT)
}
