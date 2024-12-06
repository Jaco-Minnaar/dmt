mod toml;
mod yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::Path;

use serde::Deserialize;

use crate::{ConfigError, FileError};

pub trait DatabaseConfig {
    fn db_type(&self) -> Database;
}

enum ConfigType {
    Toml,
    Yaml,
}

impl TryFrom<&Path> for ConfigType {
    type Error = ConfigError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let ext = if let Some(ext) = path.extension() {
            ext.to_str().unwrap()
        } else {
            return Err(ConfigError::UnrecognizedConfigFormat("".to_string()));
        };

        match ext {
            "yml" | "yaml" => Ok(Self::Yaml),
            "toml" => Ok(Self::Toml),
            ext => Err(ConfigError::UnrecognizedConfigFormat(ext.to_string())),
        }
    }
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

struct DmtConfigBuilder<T> {
    data: PhantomData<T>,
}

impl<T: ConfigStr> DmtConfigBuilder<T> {
    fn from_str(config_str: T) -> Self {
        Self { data: PhantomData }
    }
}

impl DmtConfig {
    pub fn from_path(path: impl AsRef<Path>) -> Result<DmtConfig, ConfigError> {
        let path = path.as_ref();

        let mut config = match ConfigType::try_from(path)? {
            ConfigType::Toml => DmtConfig,
            ConfigType::Yaml => DmtConfig,
        };

        config.resolve_env()?;
        Ok(config)
    }

    pub fn from_yaml_str(yaml: &str) -> Result<DmtConfig, ConfigError> {
        <DmtConfig as Parse<YamlConfig>>::from_str(yaml)
    }

    //pub fn from_str<T>(input: &str) -> Result<DmtConfig, ConfigError>
    //where
    //    T: ConfigFile,
    //{
    //    <DmtConfig as Parse<T>>::from_str(input)
    //}

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

trait ConfigStr {
    fn as_str(&self) -> &str;
}

trait ConfigFile {}

struct YamlConfig;
struct TomlConfig;

impl ConfigFile for YamlConfig {}
impl ConfigFile for TomlConfig {}

trait Parse<T: ConfigFile> {
    fn from_file(path: impl AsRef<Path>) -> Result<DmtConfig, ConfigError>;
    fn from_str(input: &str) -> Result<DmtConfig, ConfigError>;
}

fn default_migration_path() -> String {
    "./migrations/".to_string()
}

fn default_migration_config() -> MigrationConfig {
    MigrationConfig {
        migration_path: default_migration_path(),
    }
}
