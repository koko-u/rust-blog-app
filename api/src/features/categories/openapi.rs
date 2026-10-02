use crate::features::categories::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(
    get_categories,
    get_category,
    create_category,
    update_category,
    delete_category
))]
pub struct CategoriesApi;
