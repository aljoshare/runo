use crate::annotations::{
    charset, create_checksum, generate, generated_with_checksum, id_iter, length, needs_clone,
    needs_generation, needs_renewal, pattern,
};
use chrono::{DateTime, Utc};
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use kube::api::Patch;
use kube::{Api, ResourceExt};
use rand::Rng;

use crate::errors::{
    AnnotationUpdateError, CantCreateStringFromRegex, DataUpdateError, InvalidRegexPattern,
    SecretUpdateError,
};
use std::collections::BTreeMap;

use crate::annotations;
use crate::k8s::K8s;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::error;
use tracing::log::debug;

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
    let mut rng = rand::rng();
    let charset_b = charset.as_bytes();
    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.random_range(0..charset.len());
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
    let mut rng = rand::rng();
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

fn update_annotations(
    obj: &Arc<Secret>,
) -> Result<BTreeMap<String, String>, AnnotationUpdateError> {
    let mut secret_annotations = match &obj.metadata.annotations {
        Some(annotations) => annotations.clone(),
        None => BTreeMap::new(),
    };
    for id in id_iter(obj) {
        if !generated_with_checksum(obj, &id).exists() {
            // Migration code to make sure that old secret witout the annotations gets the update
            let generated_with_checksum_v1 = format!(
                "{}-{}",
                annotations::V1Annotation::GeneratedWithChecksum.key(),
                id
            );
            let checksum = create_checksum(obj, id.as_str());
            secret_annotations.insert(generated_with_checksum_v1, checksum);
        }
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
            let generated_with_checksum_v1 = format!(
                "{}-{}",
                annotations::V1Annotation::GeneratedWithChecksum.key(),
                id
            );
            let checksum = create_checksum(obj, id.as_str());
            secret_annotations.insert(generated_with_checksum_v1, checksum);
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
    Ok(secret_annotations)
}

fn update_data(obj: &Arc<Secret>) -> Result<BTreeMap<String, ByteString>, DataUpdateError> {
    let mut data = match &obj.data {
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
            data = update_data_field(data, obj, &id)?;
        }
        if needs_renewal(obj, id.as_str()) {
            debug!("{:?} for id {:?} needs to be renewed", obj.name_any(), id);
            data = update_data_field(data, obj, &id)?;
        }
        if needs_clone(obj, id.as_str()) {
            debug!("{:?} for id {:?} needs to get cloned", obj.name_any(), id);
            data = clone_data_field(data, obj, &id)?;
        }
    }
    Ok(data)
}

fn update_data_field(
    mut secret_data: BTreeMap<String, ByteString>,
    obj: &Arc<Secret>,
    id: &str,
) -> Result<BTreeMap<String, ByteString>, DataUpdateError> {
    let key = annotations::generate(obj, id);
    let value = generate_random_string(obj, id);
    match value {
        Ok(v) => {
            secret_data.insert(
                key.get_value().to_string(),
                ByteString(v.as_bytes().to_vec()),
            );
            Ok(secret_data)
        }
        Err(e) => {
            error!(
                "Can't generate random string for {:?}: {:?}",
                obj.name_any(),
                e
            );
            Err(DataUpdateError)
        }
    }
}

fn should_clone_already_cloned_field(obj: &Arc<Secret>, clone_from_id: &str) -> bool {
    let maybe_clone_from = annotations::clone_from(obj, clone_from_id);
    maybe_clone_from.exists()
}

fn clone_data_field(
    mut secret_data: BTreeMap<String, ByteString>,
    obj: &Arc<Secret>,
    id: &str,
) -> Result<BTreeMap<String, ByteString>, DataUpdateError> {
    let maybe_generate = annotations::generate(obj, id);
    let maybe_clone_from = annotations::clone_from(obj, id);
    let clone_from_id = maybe_clone_from.get_value();
    let clone_from_field_name = generate(obj, &clone_from_id);
    if !clone_from_field_name.exists() {
        error!(
            "Can't clone field! No annotation for field with id {:?}",
            clone_from_id
        );
        return Err(DataUpdateError);
    }
    let clone_from_field_name_value = clone_from_field_name.get_value();
    if should_clone_already_cloned_field(obj, &clone_from_id) {
        error!(
            "It's not allowed to clone an already cloned field: {:?}",
            clone_from_id
        );
        return Err(DataUpdateError);
    }
    let clone_from_field_value = secret_data.get(&clone_from_field_name_value);
    if clone_from_field_value.is_none() {
        error!(
            "Can't clone field! Data field for annotation with with id {:?} is empty",
            clone_from_id
        );
        return Err(DataUpdateError);
    };
    secret_data.insert(
        maybe_generate.get_value().to_string(),
        clone_from_field_value.unwrap().clone(),
    );
    Ok(secret_data)
}

fn get_updated_secret(obj: &Arc<Secret>) -> Result<Secret, SecretUpdateError> {
    let maybe_data = update_data(obj);
    let maybe_annotations = update_annotations(obj);
    let mut secret = Secret {
        ..Secret::default()
    };
    if maybe_data.is_err() || maybe_annotations.is_err() {
        return Err(SecretUpdateError);
    }
    secret.data = Some(maybe_data.unwrap());
    secret.metadata.annotations = Some(maybe_annotations.unwrap());
    Ok(secret)
}

pub async fn update(obj: &Arc<Secret>, k8s: &K8s) -> Result<Secret, SecretUpdateError> {
    let secrets: Api<Secret> =
        Api::namespaced(K8s::get_client().await, obj.namespace().unwrap().as_str());
    let updated_secret = get_updated_secret(obj)?;
    match secrets
        .patch(
            &obj.name_any(),
            &k8s.get_patch_params(),
            &Patch::Apply(&updated_secret),
        )
        .await
    {
        Ok(_) => Ok(updated_secret),
        Err(_) => Err(SecretUpdateError),
    }
}

#[cfg(test)]
mod tests {
    use crate::annotations::create_checksum;
    use crate::secrets::{generate_random_string, update_annotations, update_data};
    use chrono::{DateTime, Utc};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use k8s_openapi::ByteString;
    use regex::Regex;
    use rstest::rstest;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::time::SystemTime;

    use super::clone_data_field;

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

    #[rstest]
    #[case("v1.secret.runo.rocks/length-0", "1", 1)]
    #[case("v1.secret.runo.rocks/length-0", "10", 10)]
    fn test_generate_random_string_length(
        #[case] key: String,
        #[case] value: String,
        #[case] count: usize,
    ) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        debug_assert_eq!(result.chars().count(), count);
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/charset-0", "abcd")]
    fn test_generate_random_string_charset_match(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[abcd]+").unwrap();
        assert!(re.is_match(result.as_str()));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/charset-0", "abcd")]
    fn test_generate_random_string_charset_no_match(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[e-zA-Z]+").unwrap();
        assert!(!re.is_match(result.as_str()));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "\\S")]
    fn test_generate_random_string_pattern_match(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[\S]+").unwrap();
        assert!(re.is_match(result.as_str()));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "\\S")]
    fn test_generate_random_string_pattern_no_match(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let result = generate_random_string(&Arc::from(secret), "0").unwrap();
        let re = Regex::new(r"[\s]+").unwrap();
        assert!(!re.is_match(result.as_str()));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "")]
    fn test_generate_random_string_pattern_error(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
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

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/pattern-0".to_string(), "\\S".to_string())])]
    fn test_update_annotations(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        let start: DateTime<Utc> = SystemTime::now().into();
        let annotations = update_annotations(&Arc::from(secret)).unwrap();
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

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/renewal-0".to_string(), "true".to_string())])]
    fn test_update_annotations_needs_renewal(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        let annotations = update_annotations(&Arc::from(secret)).unwrap();
        assert!(annotations.contains_key("v1.secret.runo.rocks/renewal-0"));
        let needs_renewal: bool = annotations
            .get("v1.secret.runo.rocks/renewal-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_renewal);
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/renewal-0", "false")]
    fn test_update_annotations_no_need_for_renewal(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let annotations = update_annotations(&Arc::from(secret)).unwrap();
        assert!(annotations.contains_key("v1.secret.runo.rocks/renewal-0"));
        let needs_renewal: bool = annotations
            .get("v1.secret.runo.rocks/renewal-0")
            .unwrap()
            .parse()
            .unwrap();
        assert!(!needs_renewal);
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/generate-0", "username")]
    fn test_update_data(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let data = update_data(&Arc::from(secret)).unwrap();
        assert!(data.contains_key("username"));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/generate-0", "username")]
    fn test_update_annotations_creates_config_checksum(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        let annotations = update_annotations(&Arc::from(secret.clone())).unwrap();
        assert!(annotations.contains_key("v1.secret.runo.rocks/config-checksum-0"));
        let checksum = annotations
            .get("v1.secret.runo.rocks/config-checksum-0")
            .unwrap();
        assert_eq!(*checksum, create_checksum(&Arc::from(secret), "0"));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generate-1".to_string(), "username-cloned".to_string()),
    ("v1.secret.runo.rocks/clone-from-1".to_string(), "0".to_string())])]
    fn test_clone_data_field(#[case] annotations: Vec<(String, String)>) {
        let mut secret = build_secret_with_annotations(annotations);
        let mut secret_data = BTreeMap::new();
        secret_data.insert(
            "username".to_string(),
            ByteString("clone-me".as_bytes().to_vec()),
        );
        let _ = secret.data.insert(secret_data);
        assert!(secret.data.clone().unwrap().contains_key("username"));
        assert!(!secret.data.clone().unwrap().contains_key("username-cloned"));
        let result = clone_data_field(
            secret.data.clone().unwrap(),
            &Arc::from(secret.clone()),
            "1",
        );
        assert!(result.clone().unwrap().contains_key("username-cloned"));
        assert_eq!(
            result.unwrap().get("username-cloned").unwrap().0,
            "clone-me".as_bytes().to_vec()
        );
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generate-1".to_string(), "username-cloned-1".to_string()),
    ("v1.secret.runo.rocks/clone-from-1".to_string(), "0".to_string()),
    ("v1.secret.runo.rocks/generate-2".to_string(), "username-cloned-2".to_string()),
    ("v1.secret.runo.rocks/clone-from-2".to_string(), "0".to_string())])]
    fn test_clone_multiple_data_fields(#[case] annotations: Vec<(String, String)>) {
        let mut secret = build_secret_with_annotations(annotations);
        let mut secret_data = BTreeMap::new();
        secret_data.insert(
            "username".to_string(),
            ByteString("clone-me".as_bytes().to_vec()),
        );
        let _ = secret.data.insert(secret_data);
        assert!(secret.data.clone().unwrap().contains_key("username"));
        assert!(!secret.data.clone().unwrap().contains_key("username-cloned"));
        let result = clone_data_field(
            secret.data.clone().unwrap(),
            &Arc::from(secret.clone()),
            "1",
        );
        assert!(result.clone().unwrap().contains_key("username-cloned-1"));
        assert_eq!(
            result.unwrap().get("username-cloned-1").unwrap().0,
            "clone-me".as_bytes().to_vec()
        );

        let result = clone_data_field(
            secret.data.clone().unwrap(),
            &Arc::from(secret.clone()),
            "2",
        );
        assert!(result.clone().unwrap().contains_key("username-cloned-2"));
        assert_eq!(
            result.unwrap().get("username-cloned-2").unwrap().0,
            "clone-me".as_bytes().to_vec()
        );
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generate-1".to_string(), "username-cloned-1".to_string()),
    ("v1.secret.runo.rocks/clone-from-1".to_string(), "0".to_string()),
    ("v1.secret.runo.rocks/generate-2".to_string(), "username-cloned-2".to_string()),
    ("v1.secret.runo.rocks/clone-from-2".to_string(), "1".to_string())])]
    fn test_dont_clone_chained_data_fields(#[case] annotations: Vec<(String, String)>) {
        let mut secret = build_secret_with_annotations(annotations);
        let mut secret_data = BTreeMap::new();
        secret_data.insert(
            "username".to_string(),
            ByteString("clone-me".as_bytes().to_vec()),
        );
        let _ = secret.data.insert(secret_data);
        assert!(secret.data.clone().unwrap().contains_key("username"));
        assert!(!secret.data.clone().unwrap().contains_key("username-cloned"));
        let result = clone_data_field(
            secret.data.clone().unwrap(),
            &Arc::from(secret.clone()),
            "1",
        );
        assert!(result.clone().unwrap().contains_key("username-cloned-1"));
        assert_eq!(
            result.unwrap().get("username-cloned-1").unwrap().0,
            "clone-me".as_bytes().to_vec()
        );

        let result = clone_data_field(
            secret.data.clone().unwrap(),
            &Arc::from(secret.clone()),
            "2",
        );
        assert!(result.is_err());
    }
}
