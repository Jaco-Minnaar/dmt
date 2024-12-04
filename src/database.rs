mod postgres;

use chrono::NaiveDateTime;

pub use self::postgres::PostgresMigrationDatabase;

use crate::commands::Migration;

pub trait MigrationDatabase {
    fn create_migrations_table(&mut self) -> Result<(), String>;
    fn migration_table_exists(&mut self) -> Result<bool, String>;
    fn get_migrations(&mut self) -> Result<Vec<Migration>, String>;
    fn create_migration(&mut self, name: &str, time: NaiveDateTime) -> Result<Migration, String>;
    fn execute_sql(&mut self, sql: &str) -> Result<(), String>;
    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), String>;
    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), String>;
}
