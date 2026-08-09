use crate::*;

pub struct UnixAccessComponentLookup;
impl agnostic::AccessComponentLookup for UnixAccessComponentLookup {
    const LOOKUP: agnostic::AccessLookup = agnostic::AccessLookup {
        lookup_user_fn,
        lookup_group_fn,
        lookup_user_groups_fn,
        lookup_group_users_fn,
        lookup_effective_process_user_fn,
    };
}

fn lookup_user_fn(query: agnostic::AccessKeyRef<'_>) -> agnostic::BridgeResult<Option<agnostic::User>> {
    let user = match query {
        agnostic::AccessKeyRef::Name(name) => cstd_lookup_username(name)?,
        agnostic::AccessKeyRef::UnixID(id) => cstd_lookup_user(id)?,
        agnostic::AccessKeyRef::QualifiedName(_, _) => agnostic::BridgeError::err_incapable(agnostic::Capability::QualifiedAccessNames)?,
        agnostic::AccessKeyRef::WindowsSID(_) => agnostic::BridgeError::err_incapable(agnostic::Capability::WindowsSIDs)?,
    };
    
    let user = user.map(agnostic::User::from);
    Ok(user)
}

fn lookup_group_fn(query: agnostic::AccessKeyRef) -> agnostic::BridgeResult<Option<agnostic::UserGroup>> {
    let group = match query {
        agnostic::AccessKeyRef::Name(name) => cstd_lookup_groupname(name)?,
        agnostic::AccessKeyRef::UnixID(id) => cstd_lookup_group(id)?,
        agnostic::AccessKeyRef::QualifiedName(_, _) => agnostic::BridgeError::err_incapable(agnostic::Capability::QualifiedAccessNames)?,
        agnostic::AccessKeyRef::WindowsSID(_) => agnostic::BridgeError::err_incapable(agnostic::Capability::WindowsSIDs)?,
    };
    
    let group = group.map(agnostic::UserGroup::from);
    Ok(group)
}

fn lookup_effective_process_user_fn() -> agnostic::BridgeResult<agnostic::User> {
    cstd_lookup_effective_process_user().map(agnostic::User::from)
        .map_err(|e| e.into())
}

fn _lookup_cstd_real_process_user() -> agnostic::BridgeResult<agnostic::User> {
    cstd_lookup_real_process_user().map(agnostic::User::from)
        .map_err(|e| e.into())
}

fn _lookup_cstd_effective_process_group() -> agnostic::BridgeResult<agnostic::UserGroup> {
    cstd_lookup_effective_process_group().map(agnostic::UserGroup::from)
        .map_err(|e| e.into())
}

fn _lookup_cstd_real_process_group() -> agnostic::BridgeResult<agnostic::UserGroup> {
    cstd_lookup_real_process_group().map(agnostic::UserGroup::from)
        .map_err(|e| e.into())
}

fn lookup_user_groups_fn(user: &agnostic::User) -> agnostic::BridgeResult<(Vec<agnostic::UserGroup>, agnostic::Capable<agnostic::PrimaryUserGroupsCapable, agnostic::AccessKey>)> {
    let mut groups = cstd_lookup_username_secondary_groups(user.username())?
        .into_iter()
        .map(agnostic::UserGroup::from)
        .collect::<HashSet<_>>();
    
    let primary_group = cstd_lookup_user_primary_group(user.uid()?)
        .map(agnostic::UserGroup::from)?;
    
    let primary_group_key = primary_group.to_id_key();
    groups.insert(primary_group);
    
    let groups = groups.into_iter().map(agnostic::UserGroup::from).collect();
    Ok((groups, agnostic::Capable::Capable(primary_group_key)))
}

fn lookup_group_users_fn(group: &agnostic::UserGroup) -> agnostic::BridgeResult<Vec<agnostic::User>> {
    let gid = group.gid()?;
    let primary_users = cstd_lookup_group_primary_users(gid)?;
    let secondary_users = cstd_lookup_group_secondary_users(gid)?;
    
    let users = primary_users.into_iter()
        .chain(secondary_users.into_iter())
        .map(agnostic::User::from)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    
    Ok(users)
}
