use std::fs::File;

use crate::{ConfigError, FileError};

use super::{DmtConfig, Parse, YamlConfig};

impl Parse for DmtConfig {
    type Source = YamlConfig;

    fn from_file(path: impl AsRef<std::path::Path>) -> Result<DmtConfig, ConfigError> {
        let r = File::open(path).or(Err(ConfigError::FileError(FileError::NotFound)))?;

        serde_yaml::from_reader(r).or(Err(ConfigError::ParseError))
    }

    fn from_str(input: &str) -> Result<DmtConfig, ConfigError> {
        serde_yaml::from_str(input).or(Err(ConfigError::ParseError))
    }
}
