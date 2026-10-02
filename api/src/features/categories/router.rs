use axum::routing;

use crate::features::categories::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/categories",
            routing::MethodRouter::new()
                .get(handlers::get_categories)
                .post(handlers::create_category),
        )
        .route(
            "/categories/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_category)
                .put(handlers::update_category)
                .delete(handlers::delete_category),
        )
}
