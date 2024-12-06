mod commands;
mod config;
mod database;
mod io;

use std::error::Error;
use std::fmt::Display;
use std::io as stdio;
use std::process::{ExitCode, Termination};

pub use commands::{new_migration, rollback_migrations, run_migrations};
pub use config::{Database, DmtConfig, MigrationConfig};
pub use database::{DatabaseConnection, MigrationDatabase};
pub use libdmt_macros::migrate;

#[derive(Debug)]
pub enum DmtError {
    ConfigError(ConfigError),
    MigrationError(MigrationError),
}

impl Display for DmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DmtError::ConfigError(config_error) => {
                write!(f, "An error occurred reading config: {config_error}")
            }
            DmtError::MigrationError(migration_error) => {
                write!(f, "An error occurred migrating database: {migration_error}")
            }
        }
    }
}

impl Termination for DmtError {
    fn report(self) -> std::process::ExitCode {
        eprintln!("{self}");

        ExitCode::FAILURE
    }
}

impl Error for DmtError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DmtError::ConfigError(config_error) => config_error.source(),
            DmtError::MigrationError(migration_error) => migration_error.source(),
        }
    }
}

impl From<ConfigError> for DmtError {
    fn from(value: ConfigError) -> Self {
        DmtError::ConfigError(value)
    }
}

impl From<MigrationError> for DmtError {
    fn from(value: MigrationError) -> Self {
        DmtError::MigrationError(value)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    FileError(stdio::Error),
    UnrecognizedConfigFormat(String),
    ParseError(String),
}

impl From<stdio::Error> for ConfigError {
    fn from(err: stdio::Error) -> Self {
        Self::FileError(err)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileError(error) => f.write_str(error.to_string().as_str()),
            ConfigError::UnrecognizedConfigFormat(msg) => f.write_str(msg),
            ConfigError::ParseError(msg) => f.write_str(msg),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::FileError(error) => error.source(),
            ConfigError::UnrecognizedConfigFormat(_) => None,
            ConfigError::ParseError(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum MigrationError {
    FileError(stdio::Error),
    ConnectionError(ConnectionError),
}

impl From<stdio::Error> for MigrationError {
    fn from(err: stdio::Error) -> Self {
        Self::FileError(err)
    }
}

impl From<ConnectionError> for MigrationError {
    fn from(value: ConnectionError) -> Self {
        Self::ConnectionError(value)
    }
}

impl Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ConnectionError(err) => err.to_string(),
            Self::FileError(error) => error.to_string(),
        };

        f.write_str(&msg)
    }
}

impl Error for MigrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConnectionError(error) => error.source(),
            Self::FileError(error) => Some(error),
        }
    }
}

#[derive(Debug)]
pub enum ConnectionError {
    PostgresError(postgres::Error),
}

impl From<postgres::Error> for ConnectionError {
    fn from(value: postgres::Error) -> Self {
        Self::PostgresError(value)
    }
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::PostgresError(err) => err.to_string(),
        };

        f.write_str(&msg)
    }
}

impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConnectionError::PostgresError(error) => Some(error),
        }
    }
}
