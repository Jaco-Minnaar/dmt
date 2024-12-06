use libsql::Connection;

use crate::commands::Migration;
use crate::config::TursoConfig;
use crate::ConnectionError;

use super::DatabaseConnection;

pub struct TursoConnection {
    connection: Connection,
}

impl TursoConnection {
    pub fn new(config: &TursoConfig) -> Result<Self, ConnectionError> {
        todo!()
    }
}

impl DatabaseConnection for TursoConnection {
    fn create_migrations_table(&mut self) -> Result<(), ConnectionError> {
        todo!()
    }

    fn migration_table_exists(&mut self) -> Result<bool, ConnectionError> {
        todo!()
    }

    fn get_migrations(&mut self) -> Result<Vec<Migration>, ConnectionError> {
        todo!()
    }

    fn create_migration(
        &mut self,
        name: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<Migration, ConnectionError> {
        todo!()
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), ConnectionError> {
        todo!()
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), ConnectionError> {
        todo!()
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), ConnectionError> {
        todo!()
    }
}
