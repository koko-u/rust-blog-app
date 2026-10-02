use axum::extract;

use crate::errors;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::features::categories::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = models::CategoryId, Path, description = "Category Id")
    ),
    description = "Get One Category by Id",
    tag = "Categories",
    responses(
        (status = 200, description = "Category of id", body = responses::CategoryResponse),
        (status = 404, description = "the Category is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_category(
    extract::Path(id): extract::Path<models::CategoryId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::CategoryResponse>, errors::ApiError> {
    let category = repositories::select_by_id(&state.pool, id).await?;

    match category {
        Some(category) => {
            let category: models::CategoryModel = category.into();
            let category: responses::CategoryResponse = category.into();
            Ok(axum::Json(category))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Category with Id={id} not found"),
        }),
    }
}
