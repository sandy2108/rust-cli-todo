mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    }).ok_or_else(|| anyhow!("Failed to find the journal file")).ok()
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action, journal_file } = CommandLineArgs::from_args();

    let journal_file = journal_file.or_else(find_default_journal_file).expect("Failed to find the journal file");

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
