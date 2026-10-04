use axum::routing;

use crate::features::comments::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/comments",
            routing::MethodRouter::new()
                .get(handlers::get_comments_of_post)
                .post(handlers::create_comment_of_post),
        )
        .route(
            "/comments/{comment_id}",
            routing::MethodRouter::new()
                .get(handlers::get_comment)
                .put(handlers::update_comment)
                .delete(handlers::delete_comment),
        )
}
