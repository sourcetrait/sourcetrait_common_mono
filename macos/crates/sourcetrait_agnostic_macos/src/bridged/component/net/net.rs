use crate::*;

pub struct MacOsNetComponentLookup;
impl agnostic::NetComponentLookup for MacOsNetComponentLookup {
    fn lookup_hostname(&self) -> agnostic::BridgeResult<String> {
        unix::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Option<String>>> {
        Ok(agnostic::Capable::Incapable(agnostic::DomainsCapable))
    }

    fn lookup_domain_authorities(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Vec<agnostic::DomainAuthority>>> {
        Ok(agnostic::Capable::Incapable(agnostic::DomainsCapable))
    }
}
