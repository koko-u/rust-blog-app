use axum::extract;

use crate::errors;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::features::categories::requests;
use crate::features::categories::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::CategoryId, Path, description = "Category Id")
    ),
    request_body = requests::UpdateCategoryRequest,
    tag = "Categories",
    responses(
        (status = 200, description = "the Updated Category", body = responses::CategoryResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_category(
    extract::Path(id): extract::Path<models::CategoryId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateCategoryRequest>,
) -> Result<axum::Json<responses::CategoryResponse>, errors::ApiError> {
    // TODO get user_id from authentication
    let user_id = uuid::uuid!("01a106a7-4327-7287-9940-af4254498604").into();
    // validate request
    let command = request.validate_into(id, user_id, &state.pool).await?;

    let updated = shared::transaction(&state.pool, async |tx| {
        // update category
        let row = repositories::update_optional(tx, &command).await?;

        match row {
            Some(row) => Ok(models::CategoryModel::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update category, it may conflict the row"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(axum::Json(updated.into()))
}
