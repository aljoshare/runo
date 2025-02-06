use std::fmt;

#[derive(Debug, Clone)]
pub struct NoNamespaceForSecret;

impl fmt::Display for NoNamespaceForSecret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No namespace for secret")
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationsDoesntExist;

impl fmt::Display for AnnotationsDoesntExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No annotations in metadata")
    }
}

#[derive(Debug, Clone)]
pub struct CantCreateStringFromRegex;

impl fmt::Display for CantCreateStringFromRegex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't create random string from specified regex")
    }
}

#[derive(Debug, Clone)]
pub struct LogLevelMissing;

impl fmt::Display for LogLevelMissing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RUST_LOG is not set properly!")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidRegexPattern {
    pub pattern: String,
}

impl fmt::Display for InvalidRegexPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid pattern {}! You can't use quantifiers (e.g. +, *, ? or {{}}) in regex pattern",
            self.pattern
        )
    }
}

#[derive(Debug, Clone)]
pub struct DataUpdateError;

impl fmt::Display for DataUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data update failed!",)
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationUpdateError;

impl fmt::Display for AnnotationUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Annotation update failed!",)
    }
}

#[derive(Debug, Clone)]
pub struct SecretUpdateError;

impl fmt::Display for SecretUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Secret update failed!",)
    }
}
