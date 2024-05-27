use super::AppContext;
use axum::Router;

mod bikes;
mod rides;
mod users;

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
        .merge(users::router())
        .merge(bikes::router())
        .merge(rides::router())
}
