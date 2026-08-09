use crate::*;
use crate::macos::*;

pub struct MacOsCrossPlatform;
impl CrossPlatform for MacOsCrossPlatform {
    const OS: Os = Os::MacOS;
    
    fn has_terminal(&self) -> CrossResult<bool> {
        match env::var(unix::ENV_TERM) {
            Ok(_) => Ok(true),
            Err(env::VarError::NotPresent) => Ok(false),
            Err(source) => CrossError::err_env_var(unix::ENV_VAR_TERM, source),
        }
    }
    
    fn has_gui(&self) -> CrossResult<bool> {
        Ok(has_gui())
    }
    
    fn copy_file_preserved<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>,
    {
        exec_cp_preserved(source, dest)
    }
    
    fn access(&self) -> AccessComponent<'_> {
        AccessComponent(unix::libc::access_cache_lock)
    }
    
    fn run_best_editor<P>(&self, file: P, child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        let sys_open_cmd = match self.has_terminal()? {
            true => None,
            false => Some(cmd_open(file.as_ref()))
        };
        
        unix::run_best_editor(sys_open_cmd, file, child_process)
    }
    
    fn home_dir(&self) -> CrossResult<PathBuf> {
        home_dir()
    }
    
    fn init_dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        init_dir_for(base, subdir)
    }

    fn dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        dir_for(base, subdir)
    }

    fn init_xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        init_xdg_dir_for(base, subdir)
    }
    
    fn xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        xdg_dir_for(base, subdir)
    }

    fn sanitize_path<P>(&self, path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>,
    {
        Ok(path.into())
    }
    
    fn capabilities(&self) -> Capabilities {
        Capability::AccessIDs | Capability::PrimaryAccessGroups
    }
}



fn has_gui() -> bool {
}