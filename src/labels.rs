use std::sync::Arc;

use k8s_openapi::api::core::v1::Secret;

pub const MANAGED_LABEL: &str = "v1.secret.runo.rocks/managed";

pub fn managed_by_us(obj: &Arc<Secret>) -> bool {
    obj.metadata
        .labels
        .as_ref()
        .and_then(|l| l.get(MANAGED_LABEL))
        .is_some_and(|val| val == "true")
}

pub fn get_managed_label() -> String {
    MANAGED_LABEL.to_string()
}

#[cfg(test)]
mod tests {
    use super::{get_managed_label, managed_by_us};
    use k8s_openapi::api::core::v1::Secret;
    use rstest::*;
    use std::{collections::BTreeMap, sync::Arc};

    #[fixture]
    pub fn secret() -> Secret {
        Secret {
            metadata: kube::api::ObjectMeta {
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[fixture]
    pub fn managed_secret(mut secret: Secret) -> Arc<Secret> {
        let mut labels = BTreeMap::new();
        labels.insert(get_managed_label(), "true".to_string());
        secret.metadata.labels = Some(labels);
        Arc::new(secret)
    }

    #[fixture]
    pub fn unmanaged_secret_explicitly(mut secret: Secret) -> Arc<Secret> {
        let mut labels = BTreeMap::new();
        labels.insert(get_managed_label(), "false".to_string());
        secret.metadata.labels = Some(labels);
        Arc::new(secret)
    }

    #[fixture]
    pub fn unmanaged_secret_implicitly(secret: Secret) -> Arc<Secret> {
        Arc::new(secret)
    }

    #[rstest]
    fn v1_managed_secret(managed_secret: Arc<Secret>) {
        assert!(managed_by_us(&managed_secret));
    }

    #[rstest]
    fn v1_unmanaged_secret_explicitly(unmanaged_secret_explicitly: Arc<Secret>) {
        assert!(!managed_by_us(&unmanaged_secret_explicitly));
    }

    #[rstest]
    fn v1_unmanaged_secret_implicitly(unmanaged_secret_implicitly: Arc<Secret>) {
        assert!(!managed_by_us(&unmanaged_secret_implicitly));
    }
}
