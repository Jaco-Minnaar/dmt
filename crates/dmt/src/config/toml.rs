use std::fs;
use std::path::Path;

use crate::{ConfigError, FileError};

use super::{DmtConfig, FromFile, Parse, Parser};

pub struct TomlConfig {
    pub contents: String,
}

impl FromFile for TomlConfig {
    fn from_path(path: impl AsRef<Path>) -> Result<TomlConfig, ConfigError> {
        let contents =
            fs::read_to_string(path).or(Err(ConfigError::FileError(FileError::NotFound)))?;

        Ok(Self { contents })
    }
}

impl Parse<TomlConfig> for Parser {
    fn parse(&self, config: TomlConfig) -> Result<DmtConfig, ConfigError> {
        toml::from_str(&config.contents).or(Err(ConfigError::ParseError))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::{
        default_migration_path, ConnectionConfig, Database, DmtConfig, EnvConfig, MigrationConfig,
        Parse, Parser, TomlConfig, TursoConfig,
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

        let actual = Parser
            .parse(TomlConfig {
                contents: input.to_string(),
            })
            .unwrap();
        assert_eq!(actual, expected);
    }
}
