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
