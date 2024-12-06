mod postgres;
mod turso;

use chrono::NaiveDateTime;

use self::postgres::PostgresConnection;
use self::turso::TursoConnection;

use crate::commands::Migration;
use crate::{ConfigError, ConnectionError, Database, DmtConfig, DmtError, MigrationError};

pub trait DatabaseConnection {
    fn create_migrations_table(&mut self) -> Result<(), ConnectionError>;
    fn migration_table_exists(&mut self) -> Result<bool, ConnectionError>;
    fn get_migrations(&mut self) -> Result<Vec<Migration>, ConnectionError>;
    fn create_migration(
        &mut self,
        name: &str,
        time: NaiveDateTime,
    ) -> Result<Migration, ConnectionError>;
    fn execute_sql(&mut self, sql: &str) -> Result<(), ConnectionError>;
    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), ConnectionError>;
    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), ConnectionError>;
}

pub enum MigrationDatabase {
    Turso(Box<TursoConnection>),
    Postgres(Box<PostgresConnection>),
}

impl MigrationDatabase {
    fn connection(&mut self) -> &mut dyn DatabaseConnection {
        match self {
            Self::Turso(conn) => conn.as_mut(),
            Self::Postgres(conn) => conn.as_mut(),
        }
    }
}

impl TryFrom<&DmtConfig> for MigrationDatabase {
    type Error = DmtError;

    fn try_from(config: &DmtConfig) -> Result<Self, Self::Error> {
        match config.connection.database {
            Some(Database::Postgres) => {
                if let Some(config) = &config.connection.postgres {
                    Ok(Self::Postgres(Box::new(
                        PostgresConnection::new(config).map_err(MigrationError::ConnectionError)?,
                    )))
                } else {
                    Err(DmtError::ConfigError(ConfigError::ParseError(
                        "No postgres config found".to_string(),
                    )))
                }
            }
            Some(Database::Turso) => {
                if let Some(config) = &config.connection.turso {
                    Ok(Self::Turso(Box::new(
                        TursoConnection::new(config).map_err(MigrationError::ConnectionError)?,
                    )))
                } else {
                    Err(DmtError::ConfigError(ConfigError::ParseError(
                        "No postgres config found".to_string(),
                    )))
                }
            }
            None => Err(DmtError::ConfigError(ConfigError::ParseError(
                "Database type not specified in config".to_string(),
            ))),
        }
    }
}

impl DatabaseConnection for MigrationDatabase {
    fn create_migrations_table(&mut self) -> Result<(), ConnectionError> {
        self.connection().create_migrations_table()
    }

    fn migration_table_exists(&mut self) -> Result<bool, ConnectionError> {
        self.connection().migration_table_exists()
    }

    fn get_migrations(&mut self) -> Result<Vec<Migration>, ConnectionError> {
        self.connection().get_migrations()
    }

    fn create_migration(
        &mut self,
        name: &str,
        time: NaiveDateTime,
    ) -> Result<Migration, ConnectionError> {
        self.connection().create_migration(name, time)
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), ConnectionError> {
        self.connection().execute_sql(sql)
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), ConnectionError> {
        self.connection().remove_migration_by_id(id)
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), ConnectionError> {
        self.connection().remove_migration_by_name(name)
    }
}
