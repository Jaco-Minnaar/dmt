mod new_migration;
mod run_migrations;

use chrono::{DateTime, Utc};
pub use new_migration::new_migration;
pub use run_migrations::run_migrations;

struct Migration {
    id: i32,
    name: String,
    time: DateTime<Utc>,
}
