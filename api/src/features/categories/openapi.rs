use crate::features::categories::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(get_categories, create_category, update_category))]
pub struct CategoriesApi;
