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
    use crate::logging::set_logger;

    #[test]
    fn is_logger_set() {
        assert!(set_logger());
    }
}
