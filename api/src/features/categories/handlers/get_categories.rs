use axum::extract;

use crate::errors;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::features::categories::responses;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    description = "Get All Categories",
    tag = "Categories",
    responses(
        (status = 200, description = "List of Categories", body = Vec<responses::CategoryResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_categories(
    current_user: u_models::CurrentUser,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::CategoryResponse>>, errors::ApiError> {
    let categories = repositories::select_by_user_id(&state.pool, current_user.id).await?;

    let response = categories
        .into_iter()
        .map(models::CategoryModel::from)
        .map(responses::CategoryResponse::from)
        .collect();

    Ok(axum::Json(response))
}
