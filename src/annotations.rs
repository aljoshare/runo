use k8s_openapi::api::core::v1::Secret;
use kube::ResourceExt;
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Debug, Clone, Copy)]
pub struct AnnotationResult<T> {
    value: T,
    default: bool,
}

impl<T> AnnotationResult<T> {
    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn get_value(self) -> T {
        self.value
    }
}

pub fn already_generated(obj: &Arc<Secret>, id: &str) -> bool {
    let generated_at_v1 = format!("v1.secret.runo.rocks/generated-at-{}", id);
    println!("{:?}", obj.annotations().keys());
    obj.annotations().contains_key(&generated_at_v1)
}

pub fn needs_renewal(obj: &Arc<Secret>, id: &str) -> bool {
    let renewal_v1 = format!("v1.secret.runo.rocks/renewal-{}", id);
    match obj.annotations().get(&renewal_v1) {
        Some(val) => {
            debug!("Value of annotation {:?} is {:?}", renewal_v1, val);
            match val.parse() {
                Ok(bool_val) => bool_val,
                Err(e) => {
                    error!("Can't parse {} to bool, {:?}", val, e);
                    false
                }
            }
        }
        None => {
            debug!("No renewal needed {:?}", renewal_v1);
            false
        }
    }
}

pub fn has_cron(obj: &Arc<Secret>, id: &str) -> bool {
    let renewal_cron = renewal_cron(obj, id);
    !renewal_cron.is_default()
}

pub fn length(obj: &Arc<Secret>, id: &str) -> AnnotationResult<usize> {
    let length_v1 = format!("v1.secret.runo.rocks/length-{}", id);
    let default_length = 32;
    return match obj.annotations().get(&length_v1) {
        Some(value) => {
            let length = value.parse::<i32>().unwrap() as usize;
            match length > 0 && length <= 100 {
                true => AnnotationResult {
                    value: length,
                    default: false,
                },
                false => {
                    error!("Invalid length! Please set a length > 0 and <= 100. Proceeding with default length.");
                    AnnotationResult {
                        value: default_length,
                        default: true,
                    }
                }
            }
        }
        None => AnnotationResult {
            value: default_length,
            default: true,
        },
    };
}

fn _annotation_result<'a>(
    obj: &'a Arc<Secret>,
    key: String,
    default_value: &'a str,
) -> AnnotationResult<&'a str> {
    return match obj.annotations().get(&key) {
        Some(value) => AnnotationResult {
            value: value.as_str(),
            default: false,
        },
        None => AnnotationResult {
            value: default_value,
            default: true,
        },
    };
}

pub fn charset<'a>(obj: &'a Arc<Secret>, id: &str) -> AnnotationResult<&'a str> {
    let charset_v1 = format!("v1.secret.runo.rocks/charset-{}", id);
    let default_charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    _annotation_result(obj, charset_v1, default_charset)
}

pub fn pattern<'a>(obj: &'a Arc<Secret>, id: &str) -> AnnotationResult<&'a str> {
    let length_v1 = format!("v1.secret.runo.rocks/pattern-{}", id);
    let default_pattern = "[\\S]";
    _annotation_result(obj, length_v1, default_pattern)
}

#[allow(dead_code)]
pub fn generated_at<'a>(obj: &'a Arc<Secret>, id: &str) -> AnnotationResult<&'a str> {
    let generated_at_v1 = format!("v1.secret.runo.rocks/generated-at-{}", id);
    let default_generated_at = "";
    _annotation_result(obj, generated_at_v1, default_generated_at)
}

pub fn renewal_cron<'a>(obj: &'a Arc<Secret>, id: &str) -> AnnotationResult<&'a str> {
    let renewal_cron_v1 = format!("v1.secret.runo.rocks/renewal-cron-{}", id);
    let default_cron = "";
    _annotation_result(obj, renewal_cron_v1, default_cron)
}

pub fn key<'a>(obj: &'a Arc<Secret>, id: &str) -> AnnotationResult<&'a str> {
    let generate_v1 = format!("v1.secret.runo.rocks/generate-{}", id);
    let default_value = "";
    _annotation_result(obj, generate_v1, default_value)
}

pub fn id_iter(obj: &Arc<Secret>) -> Vec<String> {
    let generate_keys: Vec<_> = obj
        .annotations()
        .keys()
        .filter(|p| p.contains("v1.secret.runo.rocks/generate-"))
        .collect();
    let ids: Vec<_> = generate_keys
        .into_iter()
        .map(|p| p.replace("v1.secret.runo.rocks/generate-", ""))
        .collect();
    ids
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use std::collections::BTreeMap;

    use std::sync::Arc;
    use std::time::SystemTime;

    fn build_secret_with_annotations(annotations: Vec<(String, String)>) -> Secret {
        let annotation_map = annotations
            .into_iter()
            .collect::<BTreeMap<String, String>>();
        Secret {
            metadata: ObjectMeta {
                annotations: Some(annotation_map),
                ..ObjectMeta::default()
            },
            ..Secret::default()
        }
    }

    #[test]
    fn v1_already_generated_is_true() {
        let key_1 = String::from("v1.secret.runo.rocks/generated-at-0");
        let value_1 = String::from("true");
        let key_2 = String::from("test-annotation");
        let value_2 = String::from("true");
        let secret = build_secret_with_annotations(vec![(key_1, value_1), (key_2, value_2)]);
        assert_eq!(
            crate::annotations::already_generated(&Arc::new(secret), "0"),
            true
        );
    }

    #[test]
    fn v1_needs_renewal_is_true() {
        let key_1 = String::from("v1.secret.runo.rocks/renewal-0");
        let value_1 = String::from("true");
        let key_2 = String::from("test-annotation");
        let value_2 = String::from("true");
        let secret = build_secret_with_annotations(vec![(key_1, value_1), (key_2, value_2)]);
        assert_eq!(
            crate::annotations::needs_renewal(&Arc::new(secret), "0"),
            true
        );
    }

    #[test]
    fn v1_needs_renewal_no_valid_annotation() {
        let key_1 = String::from("v1.secret.runo.rocks/not-a-valid-annotation");
        let value_1 = String::from("true");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::needs_renewal(&Arc::new(secret), "0"),
            false
        );
    }

    #[test]
    fn v1_needs_renewal_parse_error() {
        let key_1 = String::from("v1.secret.runo.rocks/renewal-0");
        let value_1 = String::from("1");
        let key_2 = String::from("v1.secret.runo.rocks/renewal-1");
        let value_2 = String::from("True");
        let key_3 = String::from("v1.secret.runo.rocks/renewal-2");
        let value_3 = String::from("");
        let secret = build_secret_with_annotations(vec![
            (key_1, value_1),
            (key_2, value_2),
            (key_3, value_3),
        ]);
        assert_eq!(
            crate::annotations::needs_renewal(&Arc::new(secret.clone()), "0"),
            false
        );
        assert_eq!(
            crate::annotations::needs_renewal(&Arc::new(secret), "1"),
            false
        );
    }

    #[test]
    fn v1_length() {
        let key_1 = String::from("v1.secret.runo.rocks/length-0");
        let value_1 = String::from("10");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::length(&Arc::new(secret), "0").get_value(),
            10
        );
    }

    #[test]
    fn v1_length_returns_default() {
        let key_1 = String::from("v1.secret.runo.rocks/length-0");
        let value_1 = String::from("1");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::length(&Arc::new(secret), "1").is_default(),
            true
        );
    }

    #[test]
    fn v1_length_invalid() {
        let key_1 = String::from("v1.secret.runo.rocks/length-0");
        let value_1 = String::from("-1");
        let key_2 = String::from("v1.secret.runo.rocks/length-1");
        let value_2 = String::from("0");
        let key_3 = String::from("v1.secret.runo.rocks/length-2");
        let value_3 = String::from("101");
        let secret = build_secret_with_annotations(vec![
            (key_1, value_1),
            (key_2, value_2),
            (key_3, value_3),
        ]);
        assert_eq!(
            crate::annotations::length(&Arc::new(secret.clone()), "0").is_default(),
            true
        );
        assert_eq!(
            crate::annotations::length(&Arc::new(secret.clone()), "1").is_default(),
            true
        );
        assert_eq!(
            crate::annotations::length(&Arc::new(secret), "2").is_default(),
            true
        );
    }

    #[test]
    fn v1_charset() {
        let key_1 = String::from("v1.secret.runo.rocks/charset-0");
        let value_1 = String::from("abc");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::charset(&Arc::new(secret), "0").get_value(),
            "abc"
        );
    }

    #[test]
    fn v1_charset_returns_default() {
        let key_1 = String::from("v1.secret.runo.rocks/charset-0");
        let value_1 = String::from("");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::charset(&Arc::new(secret), "1").is_default(),
            true
        );
    }

    #[test]
    fn v1_pattern() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("[abc]");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::pattern(&Arc::new(secret), "0").get_value(),
            "[abc]"
        );
    }

    #[test]
    fn v1_pattern_returns_default() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::pattern(&Arc::new(secret), "1").is_default(),
            true
        );
    }

    #[test]
    fn v1_generated_at() {
        let key_1 = String::from("v1.secret.runo.rocks/generated-at-0");
        let now: DateTime<Utc> = SystemTime::now().into();
        let value_1 = now.timestamp().to_string();
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::generated_at(&Arc::new(secret), "0").get_value(),
            now.timestamp().to_string()
        );
    }

    #[test]
    fn v1_generated_at_returns_default() {
        let key_1 = String::from("v1.secret.runo.rocks/generated-at-0");
        let value_1 = String::from("");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::generated_at(&Arc::new(secret), "1").is_default(),
            true
        );
    }

    #[test]
    fn v1_has_cron_is_true() {
        let key = String::from("v1.secret.runo.rocks/renewal-cron-0");
        let value = String::from("true");
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert_eq!(crate::annotations::has_cron(&Arc::new(secret), "0"), true);
    }

    #[test]
    fn v1_cron_renewal_cron_returns_default() {
        let secret = build_secret_with_annotations(vec![]);
        assert_eq!(
            crate::annotations::renewal_cron(&Arc::new(secret), "0").is_default(),
            true
        );
    }

    #[test]
    fn v1_key() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let key_2 = String::from("v1.secret.runo.rocks/generate-1");
        let value_2 = String::from("password");
        let secret = build_secret_with_annotations(vec![(key_1, value_1), (key_2, value_2)]);
        assert_eq!(
            crate::annotations::key(&Arc::new(secret.clone()), "0").get_value(),
            "username"
        );
        assert_eq!(
            crate::annotations::key(&Arc::new(secret), "1").get_value(),
            "password"
        );
    }

    #[test]
    fn v1_key_returns_default() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        assert_eq!(
            crate::annotations::key(&Arc::new(secret), "1").is_default(),
            true
        );
    }
}
