use crate::*;

pub struct MacOsCmdComponentLookup;
impl agnostic::CmdComponentLookup for MacOsCmdComponentLookup {
    fn lookup_guess_cli_editor_open_command<P>(&self, _file: P) -> agnostic::BridgeResult<agnostic::PathGuess<'_>>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        Ok(agnostic::PathGuess {
            wherein: agnostic::WHERE_EDITOR.into(),
            which: agnostic::CLI_EDITOR_GUESSES.into()
        })
    }

    fn lookup_gui_editor_open_command<P>(&self, filepath: P) -> agnostic::BridgeResult<Command>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        self.lookup_gui_open_command(filepath)
    }

    fn lookup_gui_open_command<P>(&self, filepath: P) -> agnostic::BridgeResult<Command>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        let mut cmd = Command::new(CMD_OPEN);
        cmd.arg(filepath.as_ref());
        Ok(cmd)
    }
}
