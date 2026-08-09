use crate::*;

pub struct MacOsNetComponentLookup;
impl cross::NetComponentLookup for MacOsNetComponentLookup {
    fn lookup_hostname(&self) -> cross::BridgeResult<String> {
        unix::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> cross::BridgeResult<cross::Capable<cross::DomainsCapable, Option<String>>> {
        Ok(cross::Capable::Incapable(cross::DomainsCapable))
    }

    fn lookup_domain_authorities(&self) -> cross::BridgeResult<cross::Capable<cross::DomainsCapable, Vec<cross::DomainAuthority>>> {
        Ok(cross::Capable::Incapable(cross::DomainsCapable))
    }
}
