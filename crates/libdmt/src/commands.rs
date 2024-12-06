mod new_migration;
mod rollback_migration;
mod run_migrations;

use chrono::NaiveDateTime;
pub use new_migration::{new_migration, NewMigrationError};
pub use rollback_migration::{rollback_migrations, RollbackMigrationsError};
pub use run_migrations::{run_migrations, RunMigrationsError};

#[derive(Debug)]
pub struct Migration {
    pub id: i32,
    pub name: String,
    pub time: NaiveDateTime,
}
