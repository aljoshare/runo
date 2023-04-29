use crate::annotations::{
    get_charset, get_length, get_pattern, needs_regeneration, AnnotationResult,
};
use chrono::{DateTime, Utc};
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client, ResourceExt};
use rand::Rng;

use crate::errors::CantCreateStringFromRegex;
use std::collections::BTreeMap;

use std::sync::Arc;
use std::time::SystemTime;
use tracing::log::debug;
use tracing::{error, info};

pub fn generate_random_string(
    obj: &Arc<Secret>,
    id: &str,
) -> Result<String, CantCreateStringFromRegex> {
    let length = get_length(obj, id);
    let charset = get_charset(obj, id);
    let pattern = get_pattern(obj, id);
    let random_string = if !charset.is_default() {
        Ok(generate_random_string_from_charset(
            length.get_value(),
            charset.get_value(),
        ))
    } else {
        generate_random_string_from_pattern(length.get_value(), pattern.get_value())
    };
    debug!("Generated random string: {:?}", random_string);
    random_string
}

fn generate_random_string_from_charset(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    let charset_b = charset.as_bytes();
    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..charset.len());
            charset_b[index] as char
        })
        .collect();
    random_string
}

fn generate_random_string_from_pattern(
    length: usize,
    pattern: &str,
) -> Result<String, CantCreateStringFromRegex> {
    let mut rng = rand::thread_rng();
    let pattern_with_length = format!("{:}{{ {:},{:} }}", pattern, length, length);
    debug!("Create random string for pattern {:?}", pattern_with_length);
    let gen = rand_regex::Regex::compile(pattern_with_length.as_str(), length.try_into().unwrap());
    match gen {
        Ok(compiled) => {
            let samples = (&mut rng)
                .sample_iter(&compiled)
                .take(5)
                .collect::<Vec<String>>();
            Ok(samples.first().unwrap().to_string())
        }
        Err(e) => {
            error!("Can't create string from regex: {:?}, {:?}", pattern, e);
            Err(CantCreateStringFromRegex)
        }
    }
}

fn update_annotations(obj: &Arc<Secret>, id: &str) -> BTreeMap<String, String> {
    let mut secret_annotations = match &obj.metadata.annotations {
        Some(annotations) => annotations.clone(),
        None => BTreeMap::new(),
    };
    let generated_at_v1 = format!("v1.secret.runo.rocks/generated-at-{}", id);
    let now: DateTime<Utc> = SystemTime::now().into();
    secret_annotations.insert(generated_at_v1, now.timestamp().to_string());
    if needs_regeneration(obj, id) {
        secret_annotations.insert(
            format!("v1.secret.runo.rocks/regenerate-{}", id),
            "false".to_string(),
        );
    }
    secret_annotations
}

fn update_data(obj: &Arc<Secret>, key: &str, value: &str) -> BTreeMap<String, ByteString> {
    let mut secret_data = match &obj.data {
        Some(data) => data.clone(),
        None => BTreeMap::new(),
    };
    secret_data.insert(key.to_string(), ByteString(value.as_bytes().to_vec()));
    secret_data
}

async fn patch(obj: &Arc<Secret>, id: &str, key: &str, value: &str, client: Client) {
    let secret_name = obj.name_any();
    let secret_namespace = obj.namespace().unwrap();
    let secrets: Api<Secret> = Api::namespaced(client, secret_namespace.as_str());
    let pp = PatchParams {
        dry_run: false,
        force: true,
        field_manager: Some("runo".to_string()),
        field_validation: None,
    };
    let content = Secret {
        metadata: ObjectMeta {
            annotations: Some(update_annotations(obj, id)),
            ..ObjectMeta::default()
        },
        data: Some(update_data(obj, key, value)),
        ..Secret::default()
    };
    match secrets
        .patch(&secret_name, &pp, &Patch::Apply(&content))
        .await
    {
        Ok(_) => info!("Secret patched successfully"),
        Err(e) => error!("Can't patch secret: {:?}", e),
    }
}

pub async fn update(
    obj: &Arc<Secret>,
    id: &str,
    key: AnnotationResult<&str>,
    value: Result<String, CantCreateStringFromRegex>,
) {
    match value {
        Ok(value_string) => match Client::try_default().await {
            Ok(client) => patch(obj, id, key.get_value(), value_string.as_str(), client).await,
            Err(e) => panic!("Can't create Kubernetes client. Exiting...\n {:?}", e),
        },
        Err(e) => error!(
            "Can't generate random string for {:?}: {:?}",
            obj.name_any(),
            e
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::secrets::{generate_random_string, update_annotations, update_data};
    use chrono::{DateTime, Utc};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use k8s_openapi::ByteString;

    use regex::Regex;
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
    fn test_generate_random_string_length() {
        let key_1 = String::from("v1.secret.runo.rocks/length-0");
        let value_1 = String::from("1");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        debug_assert_eq!(result.chars().count(), 1);

        let key_1 = String::from("v1.secret.runo.rocks/length-0");
        let value_1 = String::from("10");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        debug_assert_eq!(result.chars().count(), 10);
    }

    #[test]
    fn test_generate_random_string_charset_match() {
        let key_1 = String::from("v1.secret.runo.rocks/charset-0");
        let value_1 = String::from("abcd");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[abcd]+").unwrap();
        assert!(re.is_match(result.as_str()));
    }

    #[test]
    fn test_generate_random_string_charset_no_match() {
        let key_1 = String::from("v1.secret.runo.rocks/charset-0");
        let value_1 = String::from("abcd");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[e-zA-Z]+").unwrap();
        assert!(!re.is_match(result.as_str()));
    }

    #[test]
    fn test_generate_random_string_pattern_match() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("\\S");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[\S]+").unwrap();
        assert!(re.is_match(result.as_str()));
    }

    #[test]
    fn test_generate_random_string_pattern_no_match() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("\\S");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[\s]+").unwrap();
        assert!(!re.is_match(result.as_str()));
    }

    #[test]
    fn test_generate_random_string_pattern_error() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let result = generate_random_string(&Arc::from(secret), "0");
        assert!(result.is_err())
    }

    #[test]
    fn test_update_annotations() {
        let key_1 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_1 = String::from("\\S");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let start: DateTime<Utc> = SystemTime::now().into();
        let annotations = update_annotations(&Arc::from(secret), "0");
        let end: DateTime<Utc> = SystemTime::now().into();
        assert!(annotations.contains_key("v1.secret.runo.rocks/generated-at-0"));
        let timestamp: i64 = annotations
            .get("v1.secret.runo.rocks/generated-at-0")
            .unwrap()
            .parse()
            .unwrap();
        let later_or_equal_start = timestamp >= start.timestamp();
        let before_or_equal_end = timestamp <= end.timestamp();
        assert!(later_or_equal_start);
        assert!(before_or_equal_end);
    }

    #[test]
    fn test_update_annotations_needs_regeneration() {
        let key_1 = String::from("v1.secret.runo.rocks/regenerate-0");
        let value_1 = String::from("true");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let annotations = update_annotations(&Arc::from(secret), "0");
        assert!(annotations.contains_key("v1.secret.runo.rocks/regenerate-0"));
        let needs_regeneration: bool = annotations
            .get("v1.secret.runo.rocks/regenerate-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_regeneration);
    }

    #[test]
    fn test_update_annotations_no_need_for_regeneration() {
        let key_1 = String::from("v1.secret.runo.rocks/regenerate-0");
        let value_1 = String::from("false");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let annotations = update_annotations(&Arc::from(secret), "0");
        assert!(annotations.contains_key("v1.secret.runo.rocks/regenerate-0"));
        let needs_regeneration: bool = annotations
            .get("v1.secret.runo.rocks/regenerate-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_regeneration);
    }

    #[test]
    fn test_update_data() {
        let key_1 = String::from("v1.secret.runo.rocks/regenerate-0");
        let value_1 = String::from("false");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let data = update_data(&Arc::from(secret), "username", "test");
        assert!(data.contains_key("username"));
        assert_eq!(
            data.get("username").unwrap(),
            &ByteString("test".as_bytes().to_vec())
        );
    }
}
