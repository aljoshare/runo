use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn set_logger() -> bool {
    let logger = tracing_subscriber::fmt::layer().json();
    match EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info")) {
        Ok(env_filter) => {
            let collector = Registry::default().with(logger).with(env_filter);
            tracing::subscriber::set_global_default(collector).is_ok()
        }
        Err(e) => {
            error!(
                "Can't attach logger. No additional logs will be written!: {}",
                e
            );
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logging::set_logger;
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use tracing::{info, Subscriber};
    use tracing_subscriber::Registry;

    #[test]
    fn is_logger_set() {
        assert!(set_logger());
    }
}
