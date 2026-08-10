use crate::*;

pub struct WindowsNetComponentLookup;
impl agnostic::NetComponentLookup for WindowsNetComponentLookup {
    fn lookup_hostname(&self) -> agnostic::BridgeResult<String> {
        todo!()
        //winsys::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Option<String>>> {
        agnostic::BridgeError::err_incapable(agnostic::Capability::Domains)
    }

    fn lookup_domain_authorities(&self) -> agnostic::BridgeResult<agnostic::Capable<agnostic::DomainsCapable, Vec<agnostic::DomainAuthority>>> {
        agnostic::BridgeError::err_incapable(agnostic::Capability::Domains)
    }
}
