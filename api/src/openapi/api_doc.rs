use crate::features::categories::openapi::CategoriesApi;
use crate::features::comments::openapi::CommentsApi;
use crate::features::health_check::*;
use crate::features::posts::openapi::PostsApi;
use crate::features::tags::openapi::TagsApi;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
    ok
    ),
    nest(
        (path = "/api/categories", api = CategoriesApi),
        (path = "/api/tags", api = TagsApi),
        (path = "/api/posts", api = PostsApi),
        (path = "/api/posts/{post_id}/comments", api = CommentsApi)
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;
