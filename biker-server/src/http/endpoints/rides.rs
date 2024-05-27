use crate::http::AppContext;
use axum::Router;

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
}
