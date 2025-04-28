use k8s_openapi::api::core::v1::Secret;
use kube::ResourceExt;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::{debug, error, info};

pub enum V1Annotation {
    Charset,
    Generate,
    GeneratedAt,
    GeneratedWithChecksum,
    Length,
    Pattern,
    Renewal,
    RenewalCron,
    ConfigChecksum,
    ForceOverwrite,
    CloneFrom,
}

impl V1Annotation {
    pub fn key(&self) -> String {
        match *self {
            V1Annotation::Charset => "v1.secret.runo.rocks/charset".to_string(),
            V1Annotation::Generate => "v1.secret.runo.rocks/generate".to_string(),
            V1Annotation::GeneratedAt => "v1.secret.runo.rocks/generated-at".to_string(),
            V1Annotation::GeneratedWithChecksum => {
                "v1.secret.runo.rocks/generated-with-checksum".to_string()
            }
            V1Annotation::Length => "v1.secret.runo.rocks/length".to_string(),
            V1Annotation::Pattern => "v1.secret.runo.rocks/pattern".to_string(),
            V1Annotation::Renewal => "v1.secret.runo.rocks/renewal".to_string(),
            V1Annotation::RenewalCron => "v1.secret.runo.rocks/renewal-cron".to_string(),
            V1Annotation::ConfigChecksum => "v1.secret.runo.rocks/config-checksum".to_string(),
            V1Annotation::ForceOverwrite => "v1.secret.runo.rocks/force-overwrite".to_string(),
            V1Annotation::CloneFrom => "v1.secret.runo.rocks/clone-from".to_string(),
        }
    }
    pub fn value(&self, id: &str) -> String {
        match *self {
            V1Annotation::Charset => format!("{}-{}", V1Annotation::Charset.key(), id),
            V1Annotation::Generate => format!("{}-{}", V1Annotation::Generate.key(), id),
            V1Annotation::GeneratedAt => format!("{}-{}", V1Annotation::GeneratedAt.key(), id),
            V1Annotation::GeneratedWithChecksum => {
                format!("{}-{}", V1Annotation::GeneratedWithChecksum.key(), id)
            }
            V1Annotation::Length => format!("{}-{}", V1Annotation::Length.key(), id),
            V1Annotation::Pattern => format!("{}-{}", V1Annotation::Pattern.key(), id),
            V1Annotation::Renewal => format!("{}-{}", V1Annotation::Renewal.key(), id),
            V1Annotation::RenewalCron => format!("{}-{}", V1Annotation::RenewalCron.key(), id),
            V1Annotation::ConfigChecksum => {
                format!("{}-{}", V1Annotation::ConfigChecksum.key(), id)
            }
            V1Annotation::ForceOverwrite => {
                format!("{}-{}", V1Annotation::ForceOverwrite.key(), id)
            }
            V1Annotation::CloneFrom => {
                format!("{}-{}", V1Annotation::CloneFrom.key(), id)
            }
        }
    }
    fn default(&self) -> Option<String> {
        match *self {
            V1Annotation::Charset => {
                Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string())
            }
            V1Annotation::Generate => None,
            V1Annotation::GeneratedAt => None,
            V1Annotation::GeneratedWithChecksum => None,
            V1Annotation::Length => Some("32".to_string()),
            V1Annotation::Pattern => Some("[a-zA-Z0-9\\-\\_\\(\\)\\%\\$\\@]".to_string()),
            V1Annotation::Renewal => None,
            V1Annotation::RenewalCron => None,
            V1Annotation::ConfigChecksum => None,
            V1Annotation::ForceOverwrite => Some("false".to_string()),
            V1Annotation::CloneFrom => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnnotationResult<T> {
    value: T,
    default: bool,
    exists: bool,
}

impl<T> AnnotationResult<T> {
    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn exists(&self) -> bool {
        self.exists
    }

    pub fn get_value(self) -> T {
        self.value
    }
}

fn already_set(obj: &Arc<Secret>, id: &str) -> bool {
    let generate = generate(obj, id);
    match obj.data.as_ref() {
        Some(d) => d.get(&generate.get_value()).is_some(),
        None => false,
    }
}

fn should_force_overwrite(obj: &Arc<Secret>, id: &str) -> bool {
    force_overwrite(obj, id).get_value() == "true"
}

pub fn needs_clone(obj: &Arc<Secret>, id: &str) -> bool {
    clone_from(obj, id).exists()
}

pub fn needs_generation(obj: &Arc<Secret>, id: &str) -> bool {
    if generate(obj, id).exists() {
        if needs_clone(obj, id) {
            debug!("Skip generation since field should be cloned");
            return false;
        }
        if generated_at(obj, id).exists() {
            let checksum = checksum(obj, id);
            let generated_with_checksum = generated_with_checksum(obj, id);
            if checksum.exists()
                && generated_with_checksum.exists()
                && (checksum.get_value() != generated_with_checksum.get_value())
            {
                info!("(Re)generation of secret because checksum changed");
                return true;
            }
        } else if !already_set(obj, id) {
            return true;
        } else if should_force_overwrite(obj, id) {
            info!("Overwrite existing field because annotation is set");
            return true;
        }
    }
    false
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

pub fn create_checksum(obj: &Arc<Secret>, id: &str) -> String {
    let mut hasher = Sha256::new();
    for annotation in get_annotation_values_for_id(obj, id) {
        hasher.update(annotation);
    }
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn get_annotation_values_for_id<'a>(obj: &'a Arc<Secret>, id: &'a str) -> Vec<&'a String> {
    let annotations_for_id: Vec<(&String, &String)> = obj
        .annotations()
        .iter()
        .filter(|p| !p.0.starts_with(V1Annotation::ConfigChecksum.key().as_str()))
        .filter(|p| {
            !p.0.starts_with(V1Annotation::GeneratedWithChecksum.key().as_str())
        })
        .filter(|p| !p.0.starts_with(V1Annotation::GeneratedAt.key().as_str()))
        .filter(|p| p.0.ends_with(format!("-{}", id).as_str()))
        .collect();
    annotations_for_id.iter().map(|p| p.1).collect()
}

pub fn has_cron(obj: &Arc<Secret>, id: &str) -> bool {
    let renewal_cron = renewal_cron(obj, id);
    !renewal_cron.is_default()
}

pub fn length(obj: &Arc<Secret>, id: &str) -> AnnotationResult<usize> {
    let length_v1 = V1Annotation::Length.value(id);
    match obj.annotations().get(&length_v1) {
        Some(value) => {
            let length = value.parse::<i32>().unwrap() as usize;
            match length > 0 && length <= 100 {
                true => AnnotationResult {
                    value: length,
                    default: false,
                    exists: true,
                },
                false => {
                    error!("Invalid length! Please set a length > 0 and <= 100. Proceeding with default length.");
                    match V1Annotation::Length.default() {
                        Some(default) => AnnotationResult {
                            value: default.parse::<i32>().unwrap() as usize,
                            default: true,
                            exists: false,
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
                exists: false,
            },
            None => panic!("No default set for length! Panic!"),
        },
    }
}

fn _annotation_result(
    obj: &Arc<Secret>,
    annotation: V1Annotation,
    id: &str,
) -> AnnotationResult<String> {
    match obj.annotations().get(annotation.value(id).as_str()) {
        Some(value) => AnnotationResult {
            value: value.to_string(),
            default: false,
            exists: true,
        },
        None => AnnotationResult {
            value: annotation.default().unwrap_or("".to_string()),
            default: true,
            exists: false,
        },
    }
}

pub fn charset(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Charset, id)
}

pub fn pattern(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Pattern, id)
}

pub fn generated_at(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::GeneratedAt, id)
}

pub fn generated_with_checksum(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::GeneratedWithChecksum, id)
}

pub fn renewal_cron(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::RenewalCron, id)
}

pub fn generate(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::Generate, id)
}

pub fn force_overwrite(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::ForceOverwrite, id)
}

pub fn clone_from(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::CloneFrom, id)
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

#[allow(dead_code)]
pub fn checksum(obj: &Arc<Secret>, id: &str) -> AnnotationResult<String> {
    _annotation_result(obj, V1Annotation::ConfigChecksum, id)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use k8s_openapi::ByteString;
    use rstest::*;

    use std::collections::BTreeMap;

    use crate::annotations::create_checksum;
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

    #[rstest]
    #[case("v1.secret.runo.rocks/generated-at-0", "000000000")]
    fn v1_already_generated_is_true(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(!crate::annotations::needs_renewal(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/renewal-0", "true")]
    fn v1_needs_renewal_is_true(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::needs_renewal(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/not-a-valid-annotation", "true")]
    fn v1_no_valid_annotation(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(!crate::annotations::needs_renewal(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/renewal-0", "1")]
    #[case("v1.secret.runo.rocks/renewal-0", "True")]
    #[case("v1.secret.runo.rocks/renewal-0", "")]
    fn v1_needs_renewal_parse_error(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(!crate::annotations::needs_renewal(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/length-0", "10")]
    fn v1_length(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert_eq!(
            crate::annotations::length(&Arc::new(secret), "0").get_value(),
            10
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/length-0", "1")]
    fn v1_length_returns_default(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::length(&Arc::new(secret), "1").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/length-0", "-1")]
    #[case("v1.secret.runo.rocks/length-0", "0")]
    #[case("v1.secret.runo.rocks/length-0", "101")]
    fn v1_length_invalid(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::length(&Arc::new(secret), "0").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/charset-0", "abc")]
    fn v1_charset(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert_eq!(
            crate::annotations::charset(&Arc::new(secret), "0").get_value(),
            "abc"
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/charset-0", "")]
    fn v1_charset_returns_default(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::charset(&Arc::new(secret), "1").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "[abc]")]
    fn v1_pattern(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert_eq!(
            crate::annotations::pattern(&Arc::new(secret), "0").get_value(),
            "[abc]"
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/pattern-0", "")]
    fn v1_pattern_returns_default(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::pattern(&Arc::new(secret), "1").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/generated-at-0", SystemTime::now().into())]
    fn v1_generated_at(#[case] key: String, #[case] value: DateTime<Utc>) {
        let secret = build_secret_with_annotations(vec![(key, value.to_string())]);
        assert_eq!(
            crate::annotations::generated_at(&Arc::new(secret), "0").get_value(),
            value.to_string()
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/generated-at-0", "")]
    fn v1_generated_at_returns_default(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::generated_at(&Arc::new(secret), "1").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/renewal-cron-0", "true")]
    fn v1_has_cron_is_true(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value)]);
        assert!(crate::annotations::has_cron(&Arc::new(secret), "0"));
    }

    #[rstest]
    fn v1_cron_renewal_cron_returns_default() {
        let secret = build_secret_with_annotations(vec![]);
        assert!(crate::annotations::renewal_cron(&Arc::new(secret), "0").is_default());
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/generate-0", "username")]
    #[case("v1.secret.runo.rocks/generate-0", "password")]
    fn v1_key(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value.clone())]);
        assert_eq!(
            crate::annotations::generate(&Arc::new(secret.clone()), "0").get_value(),
            value
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/config-checksum-0", "checksum")]
    fn v1_config_checksum(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value.to_string())]);
        assert_eq!(
            crate::annotations::checksum(&Arc::new(secret), "0").get_value(),
            value.to_string()
        );
    }

    #[rstest]
    #[case(
        "v1.secret.runo.rocks/generate-0",
        "username",
        "16f78a7d6317f102bbd95fc9a4f3ff2e3249287690b8bdad6b7810f82b34ace3"
    )]
    #[case(
        "v1.secret.runo.rocks/generate-0",
        "password",
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"
    )]
    fn v1_config_checksum_create(#[case] key: String, #[case] value: String, #[case] hash: String) {
        let secret = build_secret_with_annotations(vec![(key, value.to_string())]);
        let checksum = create_checksum(&Arc::new(secret.clone()), "0");
        assert_eq!(checksum, hash);
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/force-overwrite-0", "true")]
    #[case("v1.secret.runo.rocks/force-overwrite-0", "false")]
    fn v1_force_overwrite(#[case] key: String, #[case] value: bool) {
        let secret = build_secret_with_annotations(vec![(key, value.to_string())]);
        assert_eq!(
            crate::annotations::force_overwrite(&Arc::new(secret), "0").get_value(),
            value.to_string()
        );
    }

    #[rstest]
    fn v1_force_overwrite_returns_default() {
        let secret = build_secret_with_annotations(vec![]);
        assert!(crate::annotations::force_overwrite(&Arc::new(secret.clone()), "0").is_default());
        assert_eq!(
            crate::annotations::force_overwrite(&Arc::new(secret), "0").get_value(),
            "false"
        );
    }

    #[rstest]
    #[case("v1.secret.runo.rocks/clone-from-1", "0")]
    fn v1_clone_from(#[case] key: String, #[case] value: String) {
        let secret = build_secret_with_annotations(vec![(key, value.to_string())]);
        assert_eq!(
            crate::annotations::clone_from(&Arc::new(secret), "1").get_value(),
            value.to_string()
        );
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string())])]
    fn needs_generation(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        assert!(crate::annotations::needs_generation(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generated-at-0".to_string(), format!("{:?}",SystemTime::now()))])]
    fn needs_no_generation_already_generated(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        assert!(!crate::annotations::needs_generation(
            &Arc::new(secret),
            "0"
        ));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string())])]
    fn needs_no_generation_already_set(#[case] annotations: Vec<(String, String)>) {
        let mut secret = build_secret_with_annotations(annotations);
        let mut predefined_data: BTreeMap<String, ByteString> = BTreeMap::new();
        predefined_data.insert(
            "username".to_string(),
            ByteString("already-set".to_string().into_bytes()),
        );
        secret.data = Some(predefined_data);
        assert!(!crate::annotations::needs_generation(
            &Arc::new(secret),
            "0"
        ));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/force-overwrite-0".to_string(), "true".to_string())])]
    fn needs_generation_already_set_but_force_overwrite(
        #[case] annotations: Vec<(String, String)>,
    ) {
        let mut secret = build_secret_with_annotations(annotations);
        let mut predefined_data: BTreeMap<String, ByteString> = BTreeMap::new();
        predefined_data.insert(
            "username".to_string(),
            ByteString("already-set".to_string().into_bytes()),
        );
        secret.data = Some(predefined_data);
        assert!(crate::annotations::needs_generation(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generated-at-0".to_string(), format!("{:?}",SystemTime::now())),
    ("v1.secret.runo.rocks/generated-with-checksum-0".to_string(), "fghij".to_string()),
    ("v1.secret.runo.rocks/config-checksum-0".to_string(), "abcde".to_string())])]
    fn needs_generation_checksum_changed(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        assert!(crate::annotations::needs_generation(&Arc::new(secret), "0"));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generated-at-0".to_string(), format!("{:?}",SystemTime::now())),
    ("v1.secret.runo.rocks/generated-with-checksum-0".to_string(), "abcde".to_string()),
    ("v1.secret.runo.rocks/config-checksum-0".to_string(), "abcde".to_string())])]
    fn needs_no_generation_checksum_didnt_change(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        assert!(!crate::annotations::needs_generation(
            &Arc::new(secret),
            "0"
        ));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generated-at-0".to_string(), format!("{:?}",SystemTime::now())),
    ("v1.secret.runo.rocks/config-checksum-0".to_string(), "abcde".to_string())])]
    fn needs_no_generation_generated_with_checksum_doesnt_exist(
        #[case] annotations: Vec<(String, String)>,
    ) {
        let secret = build_secret_with_annotations(annotations);
        assert!(!crate::annotations::needs_generation(
            &Arc::new(secret),
            "0"
        ));
    }

    #[rstest]
    #[case(vec![("v1.secret.runo.rocks/generate-0".to_string(), "username".to_string()),
    ("v1.secret.runo.rocks/generate-1".to_string(), "username-cloned".to_string()),
    ("v1.secret.runo.rocks/clone-from-1".to_string(), "0".to_string())])]
    fn needs_no_generation_clone_from(#[case] annotations: Vec<(String, String)>) {
        let secret = build_secret_with_annotations(annotations);
        assert!(!crate::annotations::needs_generation(
            &Arc::new(secret),
            "1"
        ));
    }
}
