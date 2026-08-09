use crate::*;

pub struct LinuxNetComponentLookup;
impl agnostic::NetComponentLookup for LinuxNetComponentLookup {
    fn lookup_hostname(&self) -> agnostic::BridgeResult<String> {
        unix::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Option<String>>> {
        //let _nss_db = NssDatabase::lookup()?;
        //TODO: check parse sssd.conf
        agnostic::BridgeError::err_incapable(agnostic::Capability::Domains)
    }

    fn lookup_domain_authorities(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Vec<agnostic::DomainAuthority>>> {
        agnostic::BridgeError::err_incapable(agnostic::Capability::Domains)
    }
}
