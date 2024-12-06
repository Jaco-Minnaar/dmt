use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::ConfigError;

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct EnvConfig {
    pub file: Option<String>,
    vars: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MigrationConfig {
    #[serde(default = "default_migration_path")]
    pub migration_path: String,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct ConnectionConfig {
    pub database: Option<Database>,
    pub turso: Option<TursoConfig>,
    pub postgres: Option<PostgresConfig>,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostgresConfig {
    pub connection_string: String,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct TursoConfig {
    pub url: String,
    pub token: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Database {
    #[serde(alias = "postgres")]
    Postgres,
    #[serde(alias = "turso")]
    Turso,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DmtConfig {
    #[serde(default = "default_migration_config")]
    pub migration: MigrationConfig,
    pub connection: ConnectionConfig,
    pub env: Option<EnvConfig>,
}

impl DmtConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<DmtConfig, ConfigError> {
        let contents = fs::read_to_string(path)?;

        Self::from_str(&contents)
    }
}

impl FromStr for DmtConfig {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(|err| ConfigError::ParseError(err.to_string()))
    }
}

fn default_migration_path() -> String {
    "./migrations/".to_string()
}

fn default_migration_config() -> MigrationConfig {
    MigrationConfig {
        migration_path: default_migration_path(),
    }
}
