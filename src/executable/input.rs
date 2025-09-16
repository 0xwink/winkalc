use rustyline::{
    DefaultEditor, Config,
};

pub(super) fn new_editor() -> DefaultEditor {
    let config = Config::builder().auto_add_history(true).build();

    DefaultEditor::with_config(config).unwrap()
}