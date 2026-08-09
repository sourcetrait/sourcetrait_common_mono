use crate::*;

pub struct LinuxUiComponentLookup;
impl agnostic::UiComponentLookup for LinuxUiComponentLookup {
    fn lookup_has_command_line(&self) -> agnostic::BridgeResult<bool> {
        match env::var(unix::ENV_TERM) {
            Ok(_) => Ok(true),
            Err(env::VarError::NotPresent) => Ok(false),
            Err(source) => agnostic::BridgeError::err_env_var(unix::ENV_VAR_TERM, source),
        }
    }

    fn lookup_has_graphical(&self) -> agnostic::BridgeResult<bool> {
        match env::var(ENV_WAYLAND_DISPLAY) {
            Ok(_) => Ok(true),
            Err(env::VarError::NotPresent) => Ok(false),
            Err(source) => agnostic::BridgeError::err_env_var(ENV_WAYLAND_DISPLAY, source),
        }
    }
}