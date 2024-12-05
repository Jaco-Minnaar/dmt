mod commands;
mod config;
mod database;
mod io;

use std::io::{self as stdio, ErrorKind};

pub use config::{Database, DmtConfig, MigrationConfig};

pub use database::{DatabaseConnection, FromDatabaseConfig, MigrationDatabase};

pub use commands::{new_migration, rollback_migrations, run_migrations};
use commands::{NewMigrationError, RollbackMigrationsError, RunMigrationsError};

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

impl From<stdio::Error> for FileError {
    fn from(err: stdio::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::Other => Self::Uncategorized,
            _ => Self::Uncategorized,
        }
    }
}
