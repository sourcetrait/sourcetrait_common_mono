use crate::*;

pub struct WindowsCmdComponentLookup;
impl agnostic::CmdComponentLookup for WindowsCmdComponentLookup {
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
        const START: &'static str = "start";
        let mut cmd = Command::new(START);
        cmd.arg(filepath.as_ref());
        Ok(cmd)
    }
}
