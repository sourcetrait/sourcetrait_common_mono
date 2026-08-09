use crate::*;

pub struct MacOsUiComponentLookup;
impl cross::UiComponentLookup for MacOsUiComponentLookup {
    fn lookup_has_command_line(&self) -> cross::BridgeResult<bool> {
        match env::var(unix::ENV_TERM) {
            Ok(_) => Ok(true),
            Err(env::VarError::NotPresent) => Ok(false),
            Err(source) => cross::BridgeError::err_env_var(unix::ENV_VAR_TERM, source),
        }
    }

    fn lookup_has_graphical(&self) -> cross::BridgeResult<bool> {
        const CMD_LAUNCHCTL: &'static str = "launchctl";
        const ARG_MANAGERNAME: &'static str = "managername";
        const AQUA: &'static str = "Aqua";
        
        static HAS_GUI: LazyLock<bool> = LazyLock::new(|| {
            let cmd = Command::new(CMD_LAUNCHCTL).with_arg(ARG_MANAGERNAME);
            let name = CommandReturn::exec_for_utf8_opt(CmdKind::LaunchCtl, cmd);
            match name {
                Ok(Some(name)) if name == AQUA => true, 
                _ => false
            }
        });
        
        
        Ok(*HAS_GUI)
    }
}