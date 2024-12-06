use libsql::Connection;

use crate::config::TursoConfig;

use super::DatabaseConnection;

pub struct TursoConnection {
    connection: Connection,
}

impl TursoConnection {
    pub fn new(config: &TursoConfig) -> Result<Self, ()> {
        todo!()
    }
}

impl DatabaseConnection for TursoConnection {
    fn create_migrations_table(&mut self) -> Result<(), String> {
        todo!()
    }

    fn migration_table_exists(&mut self) -> Result<bool, String> {
        todo!()
    }

    fn get_migrations(&mut self) -> Result<Vec<crate::commands::Migration>, String> {
        todo!()
    }

    fn create_migration(
        &mut self,
        name: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<crate::commands::Migration, String> {
        todo!()
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), String> {
        todo!()
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), String> {
        todo!()
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), String> {
        todo!()
    }
}
