use axum::routing;

use crate::features::posts::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/posts",
            routing::MethodRouter::new()
                .get(handlers::get_posts)
                .post(handlers::create_post),
        )
        .route(
            "/posts/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_post)
                .put(handlers::update_post)
                .delete(handlers::delete_post),
        )
}
