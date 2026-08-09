use crate::*;

impl From<CstdError> for agnostic::BridgeError {
    fn from(e: CstdError) -> Self {
        match e {
            CstdError::NotFound { noun } => Self::NotFound { noun: noun.into() },
            CstdError::String => Self::String,
            CstdError::SysCall { source, noun } => Self::SysCall { source, noun: noun.into() },
        }
    }
}

impl From<CstdEr> for agnostic::BridgeErr {
    fn from(er: CstdEr) -> Self {
        match er {
            CstdEr::User => Self::User,
            CstdEr::UserGroup => Self::UserGroup,
            CstdEr::Hostname => Self::Hostname,
        }
    }
}