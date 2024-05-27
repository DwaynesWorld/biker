use anyhow::Context;
use clap::Parser;
use log::info;
use sqlx::postgres::PgPoolOptions;

use biker_server::config::Config;
use biker_server::{http, logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A .env file is optional
    dotenv::dotenv().ok();

    // Initialize the logger.
    // env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // Set the default log level
    logger::init(&config.log);

    info!("Starting server...");

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(25)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await?;

    // Finally, we spin up our API.
    http::serve(config, db).await?;

    Ok(())
}
