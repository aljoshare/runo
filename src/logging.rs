use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::errors::CantAttachLogger;

pub fn set_logger() -> Result<LevelFilter, CantAttachLogger> {
    let logger = tracing_subscriber::fmt::layer().json();
    match EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info")) {
        Ok(env_filter) => {
            let level = env_filter.max_level_hint().unwrap();
            let collector = Registry::default().with(logger).with(env_filter);
            if tracing::subscriber::set_global_default(collector).is_err() {
                return Err(CantAttachLogger);
            }
            Ok(level)
        }
        Err(_) => Err(CantAttachLogger),
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::logging::set_logger;
    use rstest::*;

    #[rstest]
    #[case("info")]
    fn set_valid_logger(#[case] log_level: String) {
        env::set_var("RUST_LOG", &log_level);
        let result = set_logger();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), log_level)
    }
}
