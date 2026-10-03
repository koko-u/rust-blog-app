use axum::extract;

use crate::errors;
use crate::features::tags::models;
use crate::features::tags::repositories;
use crate::features::tags::requests;
use crate::features::tags::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::TagId, Path, description = "Tag Id")
    ),
    request_body = requests::UpdateTagRequest,
    tag = "Tags",
    responses(
        (status = 200, description = "the Updated Category", body = responses::TagResponse),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_tag(
    extract::Path(id): extract::Path<models::TagId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateTagRequest>,
) -> Result<axum::Json<responses::TagResponse>, errors::ApiError> {
    // TODO get user_id from authentication
    let user_id = uuid::uuid!("c85df66a-ccd7-4f23-9df6-6accd7ff23db").into();
    // validate request
    let command = request.validate_into(id, user_id, &state.pool).await?;

    let updated = shared::transaction(&state.pool, async |tx| {
        // update category
        let row = repositories::update_optional(tx, &command).await?;

        match row {
            Some(row) => Ok(models::TagModel::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update tag, it may conflict the row"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(axum::Json(updated.into()))
}
