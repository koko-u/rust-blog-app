use axum::routing;

use crate::features::tags::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/tags",
            routing::MethodRouter::new()
                .get(handlers::get_tags)
                .post(handlers::create_tag),
        )
        .route(
            "/tags/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_tag)
                .put(handlers::update_tag)
                .delete(handlers::delete_tag),
        )
}
