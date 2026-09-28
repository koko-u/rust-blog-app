use crate::features::categories::openapi::CategoriesApi;
use crate::features::health_check::*;
use crate::features::product_brands::openapi::BrandsApi;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
    ok
    ),
    nest(
        (path = "/api/brands", api = BrandsApi),
        (path = "/api/categories", api = CategoriesApi)
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;
