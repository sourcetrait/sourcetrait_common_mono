use crate::*;

pub struct UnixPathsComponentLookup;
impl agnostic::PathsComponentLookup for UnixPathsComponentLookup {
    fn lookup_env_paths(&self) -> agnostic::BridgeResult<Vec<PathBuf>> {
        let paths = env::var(ENV_PATH)
            .map_err(|e| agnostic::BridgeError::env_var(ENV_PATH, e))?;
        let paths = paths.split(':')
            .into_iter()
            .map(|path| path.trim())
            .map(PathBuf::from)
            .collect();
        
        Ok(paths)
    }
}