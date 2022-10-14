use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{DmtError, FileError};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DmtConfig {
    pub database: Database,
    pub connection_string: String,
    pub migration_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
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
