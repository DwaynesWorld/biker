use crate::config::Config;

use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, BoxError, Router};
use hyper::StatusCode;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod endpoints;
mod error;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The core type through which handler functions can access common API state.
#[derive(Clone)]
#[allow(unused)]
pub(crate) struct AppContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let context = AppContext {
        config: Arc::new(config),
        db,
    };

    let app = Router::new()
        .merge(endpoints::router())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(context);

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
