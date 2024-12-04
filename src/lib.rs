mod cli;
mod commands;
mod config;
mod database;
mod io;

use std::io::{self as stdio, ErrorKind};

use clap::Parser;
pub use cli::Cli;
use cli::DmtCommand;
use commands::{
    new_migration, rollback_migrations, run_migrations, NewMigrationError, RollbackMigrationsError,
    RunMigrationsError,
};
use config::{Database, DmtConfig};
use database::{MigrationDatabase, PostgresMigrationDatabase};

#[derive(Debug)]
pub enum DmtError {
    ConfigError(ConfigError),
    RunMigrationError(RunMigrationsError),
    RollbackMigrationsError(RollbackMigrationsError),
    NewMigrationError(NewMigrationError),
}

#[derive(Debug)]
pub enum FileError {
    NotFound,
    MigrationsDirCouldNotCreate,
    MigrationsDirCouldNotRead,
    FileAccessDenied,
    DirAccessDenied,
    UnrecognizedConfigFormat,
    Uncategorized,
}

#[derive(Debug)]
pub enum ConfigError {
    FileError(FileError),
    UnrecognizedConfigFormat(String),
    ParseError,
}

pub fn run_dmt() {
    let cli = Cli::parse();
    let config = if let Ok(config) = config::get_config(cli.config.clone()) {
        config
    } else {
        eprintln!(
            "Could not open config file: {}",
            cli.config.to_string_lossy()
        );
        return;
    };

    let Ok(mut db) = (match config.database {
        Database::Postgres => PostgresMigrationDatabase::new(&config.connection_string),
        Database::Turso => todo!(),
    }) else {
        eprintln!("Could not establish connection to database.",);
        return;
    };

    handle_command(&cli.command, &mut db, &config).unwrap();
}

fn handle_command(
    command: &DmtCommand,
    db: &mut impl MigrationDatabase,
    config: &DmtConfig,
) -> Result<(), DmtError> {
    match command {
        DmtCommand::New(opts) => new_migration(opts, config).map_err(DmtError::NewMigrationError),
        DmtCommand::Migrate => {
            run_migrations(db, &config.migration_path).map_err(DmtError::RunMigrationError)
        }
        DmtCommand::Rollback => rollback_migrations(db, &config.migration_path)
            .map_err(DmtError::RollbackMigrationsError),
    }
}

impl From<stdio::Error> for FileError {
    fn from(err: stdio::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::Other => Self::Uncategorized,
            _ => Self::Uncategorized,
        }
    }
}
