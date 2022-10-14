mod cli;
mod commands;
mod config;

use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};

use clap::Parser;
pub use cli::Cli;
use cli::DmtCommand;
use commands::{new_migration, run_migrations};

#[derive(Debug)]
pub enum DmtError {
    FileError(FileError),
}

#[derive(Debug)]
pub enum FileError {
    NotFound,
    MigrationsDirCouldNotCreate,
    MigrationsDirCouldNotRead,
    FileAccessDenied,
    DirAccessDenied,
    Uncategorized,
}

pub fn run_dmt() {
    let cli = Cli::parse();

    handle_command(&cli)
}

fn handle_command(cli: &Cli) {
    let config = if let Ok(config) = config::get_config(cli.config.clone()) {
        config
    } else {
        eprintln!(
            "Could not open config file: {}",
            cli.config.to_string_lossy().to_string()
        );
        return;
    };

    let result = match &cli.command {
        DmtCommand::New(opts) => new_migration(opts, &config),
        DmtCommand::Migrate => run_migrations(&config),
        _ => todo!(),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
    }
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::Other => Self::Uncategorized,
            _ => Self::Uncategorized,
        }
    }
}
