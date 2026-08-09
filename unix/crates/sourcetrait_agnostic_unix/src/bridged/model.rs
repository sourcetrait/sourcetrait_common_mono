use crate::*;

impl From<UserCstd> for agnostic::User {
    fn from(u: UserCstd) -> Self {
        agnostic::User {
            ident: agnostic::AccessIdent {
                name: u.username,
                domain: agnostic::Capable::Incapable(agnostic::DomainsCapable),
                id: agnostic::Capable::Capable(u.uid),
                sid: agnostic::Capable::Incapable(agnostic::WindowsSIDsCapable),
            },
            primary_group_id: Capable::Capable(agnostic::AccessId::UnixID(u.primary_gid)),
        }
    }
}

impl From<UserGroupCstd> for agnostic::UserGroup {
    fn from(g: UserGroupCstd) -> Self {
        agnostic::UserGroup {
            ident: agnostic::AccessIdent {
                name: g.groupname,
                domain: agnostic::Capable::Incapable(agnostic::DomainsCapable),
                id: agnostic::Capable::Capable(g.gid),
                sid: agnostic::Capable::Incapable(agnostic::WindowsSIDsCapable),
            },
        }
    }
}