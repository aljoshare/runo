use tracing_subscriber::fmt::format::{DefaultFields, Format};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::errors::LogLevelMissing;

pub fn get_subscriber(
    or_default: bool,
) -> Result<FmtSubscriber<DefaultFields, Format, EnvFilter>, LogLevelMissing> {
    match EnvFilter::try_from_default_env() {
        Ok(ef) => Ok(tracing_subscriber::fmt().with_env_filter(ef).finish()),
        Err(_) => {
            if or_default {
                Ok(tracing_subscriber::fmt()
                    .with_env_filter(EnvFilter::new("INFO"))
                    .finish())
            } else {
                Err(LogLevelMissing)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logging::get_subscriber;
    use rstest::*;
    use std::env;

    #[rstest]
    #[case("ERROR")]
    #[case("WARN")]
    #[case("INFO")]
    #[case("DEBUG")]
    #[case("TRACE")]
    fn get_valid_subscriber(#[case] log_level: String) {
        temp_env::with_var("RUST_LOG", Some(&log_level), || {
            assert!(get_subscriber(false).is_ok());
            env::remove_var("RUST_LOG");
        });
    }

    #[rstest]
    fn err_if_level_not_set() {
        temp_env::with_var("RUST_LOG", None::<&str>, || {
            assert!(get_subscriber(false).is_err());
        });
    }
}
