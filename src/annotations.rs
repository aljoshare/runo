use k8s_openapi::api::core::v1::Secret;
use kube::ResourceExt;
use std::sync::Arc;
use tracing::{debug, error};

enum V1Annotation {
    Charset,
    Generate,
    GeneratedAt,
    Length,
    Pattern,
    Renewal,
    RenewalCron,
}

impl V1Annotation {
    fn key(&self) -> String {
        match *self {
            V1Annotation::Charset => "v1.secret.runo.rocks/charset".to_string(),
            V1Annotation::Generate => "v1.secret.runo.rocks/generate".to_string(),
            V1Annotation::GeneratedAt => "v1.secret.runo.rocks/generated-at".to_string(),
            V1Annotation::Length => "v1.secret.runo.rocks/length".to_string(),
            V1Annotation::Pattern => "v1.secret.runo.rocks/pattern".to_string(),
            V1Annotation::Renewal => "v1.secret.runo.rocks/renewal".to_string(),
            V1Annotation::RenewalCron => "v1.secret.runo.rocks/renewal-cron".to_string(),
        }
    }
    fn value(&self, id: &str) -> String {
        match *self {
            V1Annotation::Charset => format!("{}-{}", V1Annotation::Charset.key(), id),
            V1Annotation::Generate => format!("{}-{}", V1Annotation::Generate.key(), id),
            V1Annotation::GeneratedAt => format!("{}-{}", V1Annotation::GeneratedAt.key(), id),
            V1Annotation::Length => format!("{}-{}", V1Annotation::Length.key(), id),
            V1Annotation::Pattern => format!("{}-{}", V1Annotation::Pattern.key(), id),
            V1Annotation::Renewal => format!("{}-{}", V1Annotation::Renewal.key(), id),
            V1Annotation::RenewalCron => format!("{}-{}", V1Annotation::RenewalCron.key(), id),
        }
    }
    fn default(&self) -> Option<String> {
        match *self {
            V1Annotation::Charset => {
                Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string())
            }
            V1Annotation::Generate => None,
            V1Annotation::GeneratedAt => None,
            V1Annotation::Length => Some("32".to_string()),
            V1Annotation::Pattern => Some("[a-zA-Z0-9\\-\\_\\(\\)\\%\\$\\@]".to_string()),
            V1Annotation::Renewal => None,
            V1Annotation::RenewalCron => None,
        }
    }
}

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
    let generated_at_v1 = V1Annotation::GeneratedAt.value(id);
    println!("{:?}", obj.annotations().keys());
    obj.annotations().contains_key(&generated_at_v1)
}

pub fn needs_renewal(obj: &Arc<Secret>, id: &str) -> bool {
    let renewal_v1 = V1Annotation::Renewal.value(id);
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
    let length_v1 = V1Annotation::Length.value(id);
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
                    match V1Annotation::Length.default() {
                        Some(default) => AnnotationResult {
                            value: default.parse::<i32>().unwrap() as usize,
                            default: true,
                        },
                        None => panic!("No default set for length! Panic!"),
                    }
                }
            }
        }
        None => match V1Annotation::Length.default() {
            Some(default) => AnnotationResult {
                value: default.parse::<i32>().unwrap() as usize,
                default: true,
            },
            None => panic!("No default set for length! Panic!"),
        },
    };
}

fn _annotation_result(
    obj: &Arc<Secret>,
    annotation: V1Annotation,
    id: &str,
) -> AnnotationResult<String> {
    return match obj.annotations().get(annotation.value(id).as_str()) {
        Some(value) => AnnotationResult {
            value: value.to_string(),
            default: false,
        },
        None => AnnotationResult {
            value: annotation.default().unwrap_or("".to_string()),
            default: true,
        },
    };
}

pub fn charset(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Charset, id)
}

pub fn pattern(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Pattern, id)
}

#[allow(dead_code)]
pub fn generated_at(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::GeneratedAt, id)
}

pub fn renewal_cron(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::RenewalCron, id)
}

pub fn key(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Generate, id)
}

pub fn id_iter(obj: &Arc<Secret>) -> Vec<String> {
    let generate_keys: Vec<_> = obj
        .annotations()
        .keys()
        .filter(|p| p.contains(format!("{}-", V1Annotation::Generate.key()).as_str()))
        .collect();
    let ids: Vec<_> = generate_keys
        .into_iter()
        .map(|p| p.replace(format!("{}-", V1Annotation::Generate.key()).as_str(), ""))
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
