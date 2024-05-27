use crate::{logger::Level, BANNER};

pub const LOG: &str = "forum::server";
pub const INFO: &str = "A meeting featuring a group discussion";

/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
#[derive(clap::Parser)]
#[clap(name = "Forum server command-line interface")]
#[clap(about = INFO, before_help = BANNER, disable_version_flag = true, arg_required_else_help = true)]
pub struct Config {
    #[clap(
        long,
        env = "FORUM_LOG",
        default_value = "info",
        help = "The logging level",
        value_enum
    )]
    /// The logging level
    pub log: Level,

    /// A list of host/port pairs to use for establishing the initial connection to the Kafka cluster.
    #[clap(long, env)]
    pub bootstrap_servers: String,

    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}
