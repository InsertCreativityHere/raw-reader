
use rustyline::Editor;
use rustyline::{Completer, Helper, Highlighter, Hinter, Validator};
use rustyline::completion::FilenameCompleter;
use rustyline::config::Configurer;
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;

/// TODO
pub type DiskSelectionInputHandler = CommandLineInputHandler<DiskSelectionHelper>;

/// TODO
pub type CommandInputHandler = CommandLineInputHandler<CommandHelper>;

/// TODO
pub struct CommandLineInputHandler<H: Helper + Default> {
    editor: Editor<H, DefaultHistory>,
}

impl<H: Helper + Default> CommandLineInputHandler<H> {
    /// TODO
    pub fn new() -> Self {
        let mut editor = Editor::new().expect("Failed to create terminal handle.");
        editor.set_helper(Some(H::default()));
        editor.set_auto_add_history(true);
        CommandLineInputHandler { editor }
    }

    /// TODO
    pub fn prompt(&mut self, prompt: &str) -> String {
        self.editor.readline(prompt).expect("Failed to read from terminal.")
    }
}

/// TODO
#[derive(Helper, Completer, Hinter, Highlighter, Validator)]
pub struct DiskSelectionHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Validator)]
    validator: (),
    #[rustyline(Highlighter)]
    highlighter: (),
}

impl Default for DiskSelectionHelper {
    fn default() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            hinter: HistoryHinter {},
            validator: (),
            highlighter: (),
        }
    }
}

/// TODO
#[derive(Helper, Completer, Hinter, Highlighter, Validator)]
pub struct CommandHelper {
    #[rustyline(Completer)]
    completer: (),
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Validator)]
    validator: (),
    #[rustyline(Highlighter)]
    highlighter: (),
}

impl Default for CommandHelper {
    fn default() -> Self {
        Self {
            completer: (),
            hinter: HistoryHinter {},
            validator: (),
            highlighter: (),
        }
    }
}
