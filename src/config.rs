use crate::k8s::K8s;

#[derive(Copy, Clone)]
pub struct RunoConfig {
    pub(crate) k8s: K8s,
    pub(crate) requeue_duration: u64,
}

impl RunoConfig {
    pub fn build(k8s: K8s, requeue_duration: u64) -> RunoConfig {
        RunoConfig {
            k8s,
            requeue_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::k8s::K8s;

    use super::RunoConfig;

    #[fixture]
    fn valid_k8s() -> K8s {
        return K8s { dry_run: false };
    }

    #[rstest]
    #[case(10)]
    fn build_valid_requeue_duration(valid_k8s: K8s, #[case] requeue_duration: u64) {
        let config = RunoConfig::build(valid_k8s, requeue_duration);
        assert_eq!(config.requeue_duration, requeue_duration)
    }
}
