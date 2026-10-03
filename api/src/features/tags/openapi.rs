use crate::features::tags::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(get_tags, get_tag, create_tag, update_tag, delete_tag))]
pub struct TagsApi;
