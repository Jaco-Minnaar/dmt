use std::fs;
use std::path::Path;

use crate::{ConfigError, FileError};

use super::{DmtConfig, Parse, TomlConfig};

impl Parse for DmtConfig {
    type Source = TomlConfig;

    fn from_file(path: impl AsRef<Path>) -> Result<DmtConfig, ConfigError> {
        let contents =
            fs::read_to_string(path).or(Err(ConfigError::FileError(FileError::NotFound)))?;

        Self::from_str(&contents)
    }

    fn from_str(input: &str) -> Result<DmtConfig, ConfigError> {
        toml::from_str(&input).or(Err(ConfigError::ParseError))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::{
        default_migration_path, ConnectionConfig, Database, DmtConfig, EnvConfig, MigrationConfig,
        Parse, TomlConfig, TursoConfig,
    };

    #[test]
    fn parse_toml_valid_input() {
        let input = r#"
            [connection]
            database = "turso"
            
            [connection.turso]
            url = "test_url"
            token = "test_token"
            
            [env.vars]
            TEST_VAR = "This is a test var"
            
        "#;

        let expected = DmtConfig {
            migration: MigrationConfig {
                migration_path: default_migration_path(),
            },
            connection: ConnectionConfig {
                database: Some(Database::Turso),
                turso: Some(TursoConfig {
                    url: "test_url".to_string(),
                    token: "test_token".to_string(),
                }),
                ..Default::default()
            },
            env: Some(EnvConfig {
                vars: Some(HashMap::from_iter([(
                    "TEST_VAR".to_string(),
                    "This is a test var".to_string(),
                )])),
                ..Default::default()
            }),
        };

        let actual = <DmtConfig as Parse<TomlConfig>>::from_str(input).unwrap();
        assert_eq!(actual, expected);
    }
}
