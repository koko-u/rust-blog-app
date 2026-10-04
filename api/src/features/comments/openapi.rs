use crate::features::comments::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(
    get_comments_of_post,
    get_comment,
    create_comment_of_post,
    update_comment,
    delete_comment
))]
pub struct CommentsApi;
