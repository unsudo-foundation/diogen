use super::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq)]
pub enum Url {
    External(String),
    Internal(Asset)
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Url {
    fn to_string(&self) -> String {
        match self {
            Self::External(s) => s.to_string(),
            Self::Internal(a) => a.to_string()
        }
    }
}