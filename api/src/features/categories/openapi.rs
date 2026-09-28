use crate::features::categories::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(get_categories))]
pub struct CategoriesApi;
