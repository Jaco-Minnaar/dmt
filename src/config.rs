use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{DmtError, FileError};

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
}

pub fn get_config(path: PathBuf) -> Result<DmtConfig, DmtError> {
    let contents =
        fs::read_to_string(path.clone()).or(Err(DmtError::FileError(FileError::NotFound)))?;

    let config: DmtConfig = serde_yaml::from_str(contents.as_str()).unwrap();

    Ok(config)
}

fn default_migration_path() -> PathBuf {
    PathBuf::from("./migrations/")
}
