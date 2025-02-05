use crate::annotations::{
    charset, create_checksum, id_iter, length, needs_generation, needs_renewal, pattern,
};
use chrono::{DateTime, Utc};
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use kube::api::Patch;
use kube::{Api, ResourceExt};
use rand::Rng;

use crate::errors::{CantCreateStringFromRegex, InvalidRegexPattern};
use std::collections::BTreeMap;

use crate::annotations;
use crate::k8s::K8s;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::log::debug;
use tracing::{error, info};

pub fn generate_random_string(
    obj: &Arc<Secret>,
    id: &str,
) -> Result<String, CantCreateStringFromRegex> {
    let length = length(obj, id);
    let charset = charset(obj, id);
    let pattern = pattern(obj, id);
    let random_string = if !charset.is_default() {
        Ok(generate_random_string_from_charset(
            length.get_value(),
            charset.get_value().as_str(),
        ))
    } else {
        match validate_pattern(pattern.get_value().as_str()) {
            Ok(p) => generate_random_string_from_pattern(length.get_value(), p),
            Err(e) => {
                error!("{}", e);
                Err(CantCreateStringFromRegex)
            }
        }
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

fn validate_pattern(pattern: &str) -> Result<&str, InvalidRegexPattern> {
    let forbidden_chars = vec!["+", "?", "*", "{", "}"];
    for char in forbidden_chars {
        if pattern.contains(char) {
            return Err(InvalidRegexPattern {
                pattern: pattern.to_string(),
            });
        }
    }
    Ok(pattern)
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

fn update_annotations(obj: &Arc<Secret>) -> BTreeMap<String, String> {
    let mut secret_annotations = match &obj.metadata.annotations {
        Some(annotations) => annotations.clone(),
        None => BTreeMap::new(),
    };
    for id in id_iter(obj) {
        if needs_generation(obj, id.as_str()) {
            debug!(
                "{:?} annotations for id {:?} will be updated",
                obj.name_any(),
                id
            );
            let generated_at_v1 =
                format!("{}-{}", annotations::V1Annotation::GeneratedAt.key(), id);
            let now: DateTime<Utc> = SystemTime::now().into();
            secret_annotations.insert(generated_at_v1, now.timestamp().to_string());
        }
        if needs_renewal(obj, id.as_str()) {
            secret_annotations.insert(
                format!("{}-{}", annotations::V1Annotation::Renewal.key(), id),
                "false".to_string(),
            );
        }
        let checksum = create_checksum(obj, id.as_str());
        debug!("Adding checksum {:?} for config with ID {:?}", checksum, id);
        let checksum_v1: String =
            format!("{}-{}", annotations::V1Annotation::ConfigChecksum.key(), id);
        secret_annotations.insert(checksum_v1, checksum);
    }
    secret_annotations
}

fn update_data(obj: &Arc<Secret>) -> BTreeMap<String, ByteString> {
    let mut secret_data = match &obj.data {
        Some(data) => data.clone(),
        None => BTreeMap::new(),
    };
    for id in id_iter(obj) {
        if needs_generation(obj, id.as_str()) {
            debug!(
                "{:?} data for id {:?} will be generated",
                obj.name_any(),
                id
            );
            secret_data = update_data_field(secret_data, obj, &id);
        }
        if needs_renewal(obj, id.as_str()) {
            debug!("{:?} for id {:?} needs to be renewed", obj.name_any(), id);
            secret_data = update_data_field(secret_data, obj, &id);
        }
    }
    secret_data
}

fn update_data_field(
    mut secret_data: BTreeMap<String, ByteString>,
    obj: &Arc<Secret>,
    id: &str,
) -> BTreeMap<String, ByteString> {
    let key = annotations::generate(obj, id);
    let value = generate_random_string(obj, id);
    match value {
        Ok(v) => {
            secret_data.insert(
                key.get_value().to_string(),
                ByteString(v.as_bytes().to_vec()),
            );
            secret_data
        }
        Err(e) => {
            error!(
                "Can't generate random string for {:?}: {:?}",
                obj.name_any(),
                e
            );
            secret_data
        }
    }
}

fn get_updated_secret(obj: &Arc<Secret>) -> Secret {
    Secret {
        metadata: ObjectMeta {
            annotations: Some(update_annotations(obj)),
            ..ObjectMeta::default()
        },
        data: Some(update_data(obj)),
        ..Secret::default()
    }
}

pub async fn update(obj: &Arc<Secret>, k8s: &K8s) {
    let secrets: Api<Secret> =
        Api::namespaced(K8s::get_client().await, obj.namespace().unwrap().as_str());
    match secrets
        .patch(
            &obj.name_any(),
            &k8s.get_patch_params(),
            &Patch::Apply(&get_updated_secret(obj)),
        )
        .await
    {
        Ok(_) => info!("Secret reconciled successfully"),
        Err(e) => error!("Couldn't reconcile secret: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::annotations::create_checksum;
    use crate::secrets::{generate_random_string, update_annotations, update_data};
    use chrono::{DateTime, Utc};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use regex::Regex;
    use rstest::rstest;
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

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "[abcd]+")]
    #[case("v1.secret.runo.rocks/pattern-0", "[abcd]?")]
    #[case("v1.secret.runo.rocks/pattern-0", "[abcd]*")]
    #[case("v1.secret.runo.rocks/pattern-0", "[abcd]{1, 10}")]
    fn test_generate_random_string_pattern_invalid(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0");
        assert!(result.is_err())
    }

    #[test]
    fn test_update_annotations() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let key_2 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_2 = String::from("\\S");
        let secret = build_secret_with_annotations(vec![(key_1, value_1), (key_2, value_2)]);
        let start: DateTime<Utc> = SystemTime::now().into();
        let annotations = update_annotations(&Arc::from(secret));
        let end: DateTime<Utc> = SystemTime::now().into();
        assert!(annotations.contains_key("v1.secret.runo.rocks/generated-at-0"));
        assert!(annotations.contains_key("v1.secret.runo.rocks/config-checksum-0"));
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
    fn test_update_annotations_needs_renewal() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let key_2 = String::from("v1.secret.runo.rocks/renewal-0");
        let value_2 = String::from("true");
        let secret = build_secret_with_annotations(vec![(key_1, value_1), (key_2, value_2)]);
        let annotations = update_annotations(&Arc::from(secret));
        assert!(annotations.contains_key("v1.secret.runo.rocks/renewal-0"));
        let needs_renewal: bool = annotations
            .get("v1.secret.runo.rocks/renewal-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_renewal);
    }

    #[test]
    fn test_update_annotations_no_need_for_renewal() {
        let key_1 = String::from("v1.secret.runo.rocks/renewal-0");
        let value_1 = String::from("false");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let annotations = update_annotations(&Arc::from(secret));
        assert!(annotations.contains_key("v1.secret.runo.rocks/renewal-0"));
        let needs_renewal: bool = annotations
            .get("v1.secret.runo.rocks/renewal-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_renewal);
    }

    #[test]
    fn test_update_data() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let data = update_data(&Arc::from(secret));
        assert!(data.contains_key("username"));
        assert!(data.get("username").is_some())
    }

    #[test]
    fn test_update_annotations_creates_config_checksum() {
        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");
        let secret = build_secret_with_annotations(vec![(key_1, value_1)]);
        let annotations = update_annotations(&Arc::from(secret.clone()));
        assert!(annotations.contains_key("v1.secret.runo.rocks/config-checksum-0"));
        let checksum = annotations
            .get("v1.secret.runo.rocks/config-checksum-0")
            .unwrap();
        assert_eq!(*checksum, create_checksum(&Arc::from(secret), "0"));
    }
}
