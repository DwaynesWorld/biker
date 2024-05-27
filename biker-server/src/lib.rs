#[macro_use]
extern crate log;

pub mod config;
pub mod http;
pub mod logger;

pub mod protos {
    pub mod types {
        tonic::include_proto!("biker.types");
    }

    pub mod api {
        pub mod v1 {
            pub mod users {
                tonic::include_proto!("biker.api.users.v1");
            }
            pub mod bikes {
                tonic::include_proto!("biker.api.bikes.v1");
            }
            pub mod rides {
                tonic::include_proto!("biker.api.rides.v1");
            }
            pub mod payments {
                tonic::include_proto!("biker.api.payments.v1");
            }
        }
    }
}

pub const BANNER: &str = "BIKER";

// The name and version of this build
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERS: &str = env!("CARGO_PKG_VERSION");
pub const RUST_VERS: &str = env!("RUST_VERSION");
pub const GIT_VERS: &str = env!("GIT_VERSION");
pub const GIT_BRANCH: &str = env!("GIT_BRANCH");
pub const GIT_SHA: &str = env!("GIT_SHA");
