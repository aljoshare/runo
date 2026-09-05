use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("No namespace for secret")]
pub struct NoNamespaceForSecret;

#[derive(Error, Debug, Clone)]
#[error("Can't create random string from specified regex")]
pub struct CantCreateStringFromRegex;

#[derive(Error, Debug, Clone)]
#[error("RUST_LOG is not set properly!")]
pub struct LogLevelMissing;

#[derive(Error, Debug, Clone)]
#[error(
    "Invalid pattern {pattern}! You can't use quantifiers (e.g. +, *, ? or {{}}) in regex pattern"
)]
pub struct InvalidRegexPattern {
    pub pattern: String,
}

#[derive(Error, Debug, Clone)]
#[error("Data update failed!")]
pub struct DataUpdateError;

#[derive(Error, Debug, Clone)]
#[error("Annotation update failed!")]
pub struct AnnotationUpdateError;

#[derive(Error, Debug, Clone)]
#[error("Secret update failed!")]
pub struct SecretUpdateError;

#[derive(Error, Debug, Clone)]
#[error("Duplicate keys detected: {}", duplicates.join(", "))]
pub struct DuplicateKeysError {
    pub duplicates: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_displays() {
        assert_eq!(NoNamespaceForSecret.to_string(), "No namespace for secret");
        assert_eq!(
            CantCreateStringFromRegex.to_string(),
            "Can't create random string from specified regex"
        );
        assert_eq!(LogLevelMissing.to_string(), "RUST_LOG is not set properly!");
        assert_eq!(
            InvalidRegexPattern {
                pattern: "[a-z]+".to_string()
            }
            .to_string(),
            "Invalid pattern [a-z]+! You can't use quantifiers (e.g. +, *, ? or {}) in regex pattern"
        );
        assert_eq!(DataUpdateError.to_string(), "Data update failed!");
        assert_eq!(
            AnnotationUpdateError.to_string(),
            "Annotation update failed!"
        );
        assert_eq!(SecretUpdateError.to_string(), "Secret update failed!");
        assert_eq!(
            DuplicateKeysError {
                duplicates: vec!["key1".to_string(), "key2".to_string()]
            }
            .to_string(),
            "Duplicate keys detected: key1, key2"
        );
    }
}
