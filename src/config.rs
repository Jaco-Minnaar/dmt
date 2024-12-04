use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{ConfigError, FileError};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DmtConfig {
    pub database: Database,
    pub connection_string: String,

    #[serde(default = "default_migration_path")]
    pub migration_path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub enum Database {
    #[serde(alias = "postgres")]
    Postgres,
    #[serde(alias = "turso")]
    Turso,
}

pub fn get_config(path: PathBuf) -> Result<DmtConfig, ConfigError> {
    let ext = if let Some(ext) = path.extension() {
        ext.to_string_lossy().into_owned()
    } else {
        return Err(ConfigError::UnrecognizedConfigFormat("".to_string()));
    };

    let contents = fs::read_to_string(path).or(Err(ConfigError::FileError(FileError::NotFound)))?;

    let config: DmtConfig = match ext.as_str() {
        "yml" | "yaml" => {
            serde_yaml::from_str(contents.as_str()).or(Err(ConfigError::ParseError))?
        }
        "toml" => toml::from_str(contents.as_str()).or(Err(ConfigError::ParseError))?,
        ext => return Err(ConfigError::UnrecognizedConfigFormat(ext.to_string())),
    };

    Ok(config)
}

fn default_migration_path() -> PathBuf {
    PathBuf::from("./migrations/")
}
