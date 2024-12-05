mod postgres;
mod turso;

use chrono::NaiveDateTime;

use self::postgres::PostgresConnection;
use self::turso::TursoConnection;

use crate::commands::Migration;
use crate::config::{DatabaseConfig, PostgresConfig, TursoConfig};

pub trait DatabaseConnection {
    fn create_migrations_table(&mut self) -> Result<(), String>;
    fn migration_table_exists(&mut self) -> Result<bool, String>;
    fn get_migrations(&mut self) -> Result<Vec<Migration>, String>;
    fn create_migration(&mut self, name: &str, time: NaiveDateTime) -> Result<Migration, String>;
    fn execute_sql(&mut self, sql: &str) -> Result<(), String>;
    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), String>;
    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), String>;
}

pub trait FromDatabaseConfig<T>
where
    T: DatabaseConfig,
{
    fn from_database_config(config: &T) -> Result<Self, ()>
    where
        Self: Sized;
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

impl FromDatabaseConfig<TursoConfig> for MigrationDatabase {
    fn from_database_config(config: &TursoConfig) -> Result<Self, ()> {
        Ok(Self::Turso(Box::new(TursoConnection::new(config)?)))
    }
}

impl FromDatabaseConfig<PostgresConfig> for MigrationDatabase {
    fn from_database_config(config: &PostgresConfig) -> Result<Self, ()> {
        Ok(Self::Postgres(Box::new(PostgresConnection::new(config)?)))
    }
}

impl DatabaseConnection for MigrationDatabase {
    fn create_migrations_table(&mut self) -> Result<(), String> {
        self.connection().create_migrations_table()
    }

    fn migration_table_exists(&mut self) -> Result<bool, String> {
        self.connection().migration_table_exists()
    }

    fn get_migrations(&mut self) -> Result<Vec<Migration>, String> {
        self.connection().get_migrations()
    }

    fn create_migration(&mut self, name: &str, time: NaiveDateTime) -> Result<Migration, String> {
        self.connection().create_migration(name, time)
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), String> {
        self.connection().execute_sql(sql)
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), String> {
        self.connection().remove_migration_by_id(id)
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), String> {
        self.connection().remove_migration_by_name(name)
    }
}
