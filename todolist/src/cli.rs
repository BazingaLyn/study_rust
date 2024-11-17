use std::fmt::{Display, Formatter};
use clap::{arg, crate_description, crate_name, crate_version, Command, ValueEnum};
#[derive(ValueEnum, Clone, Copy)]
pub(crate) enum Language {
    En,
    Zh,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}


#[derive(ValueEnum, Clone, Copy)]
pub(crate) enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Display for Shell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

pub(crate) const ADD: &str = "add";
pub(crate) const CLEAR: &str = "clear";
pub(crate) const COMPLETION: &str = "completion";
pub(crate) const CONTINUE: &str = "continue";
pub(crate) const DELETE: &str = "delete";
pub(crate) const DONE: &str = "done";
pub(crate) const EDIT: &str = "edit";
pub(crate) const LIST: &str = "list";
pub(crate) const RECORD: &str = "record";
pub(crate) const REPORT: &str = "report";
pub(crate) const SHOW: &str = "show";
pub(crate) const SORT: &str = "sort";
pub(crate) const SWAP: &str = "swap";
pub(crate) const UNCONTINUE: &str = "uncontinue";
pub(crate) const UNDONE: &str = "undone";
pub(crate) const UNRECORD: &str = "unrecord";

pub(crate) fn build() -> Command {
    Command::new(crate_name!()).version(crate_version!()).about(crate_description!())
        .subcommand_required(true).arg_required_else_help(true)
        .subcommand(Command::new(LIST).about("show todo list"))
        .subcommand(Command::new(CLEAR).about("clear todo list"))
        .subcommand(Command::new(ADD).about("add the task").arg(arg!(<TASK>).required(true)))
}