use std::sync::Arc;

use k8s_openapi::api::core::v1::Secret;

pub fn managed_by_us(obj: &Arc<Secret>) -> bool {
    let r = match &obj.metadata.labels {
        Some(l) => match l.get(&get_managed_label()) {
            Some(m) => m == &"true".to_string(),
            None => {
                return false;
            }
        },
        None => false,
    };
    r
}

pub fn get_managed_label() -> String {
    "v1.secret.runo.rocks/managed".to_string()
}
