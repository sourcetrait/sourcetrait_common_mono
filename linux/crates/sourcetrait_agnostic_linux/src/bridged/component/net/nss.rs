use crate::*;

/// Supported NSS service specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum NssServiceSpec {
    SSSD,
    WinBind,
}

/// An NSS config that contains at least one supported [NssServiceSpec]
/// in either groups or users, indicating [Capability::Domains].
#[derive(Debug)]
pub(crate) struct NssDatabase {
    groups: Vec<NssServiceSpec>,
    users: Vec<NssServiceSpec>,
}

impl NssDatabase {
    pub(crate) fn lookup() -> agnostic::BridgeResult<agnostic::Capable<NssDatabase>> {
        lookup_nss_database()
    }
    
    pub(crate) fn contains_spec(&self, spec: NssServiceSpec) -> bool {
        self.groups.contains(&spec) || self.users.contains(&spec)
    }
}

fn lookup_nss_database() -> CrossResult<Capable<NssDatabase>> {
    const NSSWITCH_CONF_PATH: &'static str = "/etc/nsswitch.conf";
    const GROUPS_PREFIX: &'static str = "group:";
    const PASSWD_PREFIX: &'static str = "passwd:";
    
    let conf = fs::read_to_string(NSSWITCH_CONF_PATH)
        .map_err(|_| CrossError::incapable(Capability::Domains))?;
    let mut lines = conf.lines().into_iter()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with('#'));
    
    fn collect_specs(line: &str, prefix: &'static str) -> Vec<NssServiceSpec> {
        line.trim_start_matches(prefix)
            .trim_start()
            .split_whitespace()
            .into_iter()
            .filter_map(|s| NssServiceSpec::from_str(s).ok())
            .collect()
    }
    
    let (mut groups, mut users) = (None, None);
    while (groups.is_none() || users.is_none()) && let Some(line) = lines.next() {
        if groups.is_none() && line.starts_with(GROUPS_PREFIX) {
            groups = Some(collect_specs(line, GROUPS_PREFIX));
        } else if users.is_none() && line.starts_with(PASSWD_PREFIX) {
            users = Some(collect_specs(line, PASSWD_PREFIX));
        }
    }
    
    if groups.as_ref().is_none_or(|vec| vec.is_empty())
        && users.as_ref().is_none_or(|vec| vec.is_empty())
    {
        return CrossError::err_incapable(Capability::Domains);
    }
    
    Ok(Capable::Capable(NssDatabase {
        groups: groups.unwrap_or_default(),
        users: users.unwrap_or_default(),
    }))
}