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
