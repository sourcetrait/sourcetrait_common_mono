use crate::*;

pub fn lookup_hostname() -> agnostic::BridgeResult<String> {
    cstd_lookup_hostname().map_err(agnostic::BridgeError::from)
}

