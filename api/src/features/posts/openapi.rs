use crate::features::posts::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(get_posts, get_post, create_post, update_post, delete_post))]
pub struct PostsApi;
