mod new_migration;
mod rollback_migration;
mod run_migrations;

use chrono::NaiveDateTime;
pub use new_migration::new_migration;
pub use rollback_migration::rollback_migrations;
pub use run_migrations::run_migrations;

#[derive(Debug)]
pub struct Migration {
    pub id: i32,
    pub name: String,
    pub time: NaiveDateTime,
}
