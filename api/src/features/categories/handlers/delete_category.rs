use axum::extract;
use axum::http;

use crate::errors;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = models::CategoryId, Path, description = "Category Id")
    ),
    description = "Delete Category by Id",
    tag = "Categories",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "the Product not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn delete_category(
    extract::Path(id): extract::Path<models::CategoryId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    let deleted_brand = shared::transaction(&state.pool, async |tx| {
        let row = repositories::delete_by_id(tx, id).await?;

        match row {
            Some(row) => Ok(models::CategoryModel::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("id"),
                    garde::Error::new(format!("Category id='{id}' not found")),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(http::StatusCode::NO_CONTENT)
}
