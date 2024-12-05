mod toml;
mod yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::Deserialize;

use crate::{ConfigError, FileError};

use self::toml::TomlConfig;
use self::yaml::YamlConfig;

pub trait DatabaseConfig {
    fn db_type(&self) -> Database;
}

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

impl DatabaseConfig for PostgresConfig {
    fn db_type(&self) -> Database {
        Database::Postgres
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct TursoConfig {
    pub url: String,
    pub token: String,
}

impl DatabaseConfig for TursoConfig {
    fn db_type(&self) -> Database {
        Database::Turso
    }
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
    pub fn from_path(path: impl AsRef<Path>) -> Result<DmtConfig, ConfigError> {
        let config_file = ConfigFile::from_path(path)?;
        let mut config = Parser.parse(config_file)?;

        config.resolve_env()?;
        Ok(config)
    }

    fn resolve_env(&mut self) -> Result<(), ConfigError> {
        let Some(env) = &mut self.env else {
            return Ok(());
        };

        let env_vars = if let Some(vars) = env.vars.as_mut() {
            vars
        } else {
            &mut HashMap::new()
        };

        if let Some(file) = env.file.as_ref() {
            let reader = BufReader::new(
                File::open(file).or(Err(ConfigError::FileError(FileError::NotFound)))?,
            );

            for (no, line) in reader.lines().enumerate() {
                let Ok(line) = line else {
                    return Err(ConfigError::FileError(FileError::Uncategorized));
                };

                if line.trim().is_empty() {
                    continue;
                }

                let mut parts = line.split('=');

                match (parts.next(), parts.next()) {
                    (Some(name), Some(value)) => {
                        env_vars.insert(name.trim().to_string(), value.trim().to_string())
                    }
                    (None, _) => {
                        eprintln!("Invalid content in env file on line {}", no + 1);
                        continue;
                    }
                    (_, None) => continue,
                };
            }
        }

        Ok(())
    }
}

trait FromFile {
    fn from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError>
    where
        Self: Sized;
}

enum ConfigFile {
    Yaml(YamlConfig),
    Toml(TomlConfig),
}

trait Parse<T> {
    fn parse(&self, config: T) -> Result<DmtConfig, ConfigError>;
}

impl FromFile for ConfigFile {
    fn from_path(path: impl AsRef<Path>) -> Result<ConfigFile, ConfigError> {
        let path = path.as_ref();
        let ext = if let Some(ext) = path.extension() {
            ext.to_str().unwrap()
        } else {
            return Err(ConfigError::UnrecognizedConfigFormat("".to_string()));
        };

        match ext {
            "yml" | "yaml" => Ok(Self::Yaml(YamlConfig::from_path(path)?)),
            "toml" => Ok(Self::Toml(TomlConfig::from_path(path)?)),
            ext => Err(ConfigError::UnrecognizedConfigFormat(ext.to_string())),
        }
    }
}

struct Parser;

impl Parse<ConfigFile> for Parser {
    fn parse(&self, config: ConfigFile) -> Result<DmtConfig, ConfigError> {
        match config {
            ConfigFile::Yaml(yaml_config) => Parser.parse(yaml_config),
            ConfigFile::Toml(toml_config) => Parser.parse(toml_config),
        }
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
