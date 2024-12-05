use std::fs::File;
use std::path::Path;

use crate::{ConfigError, FileError};

use super::{DmtConfig, FromFile, Parse, Parser};

pub struct YamlConfig {
    file: File,
}

impl FromFile for YamlConfig {
    fn from_path(path: impl AsRef<Path>) -> Result<YamlConfig, ConfigError> {
        let r = File::open(path).or(Err(ConfigError::FileError(FileError::NotFound)))?;

        Ok(Self { file: r })
    }
}

impl Parse<YamlConfig> for Parser {
    fn parse(&self, config: YamlConfig) -> Result<DmtConfig, ConfigError> {
        serde_yaml::from_reader(config.file).or(Err(ConfigError::ParseError))
    }
}
