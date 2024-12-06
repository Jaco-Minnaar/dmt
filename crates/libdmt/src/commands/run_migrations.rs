use std::path::Path;

use chrono::Utc;

use crate::MigrationError;
use crate::{database::DatabaseConnection, io::MigrationDir};

pub fn run_migrations(
    db: &mut impl DatabaseConnection,
    path: &impl AsRef<Path>,
) -> Result<(), MigrationError> {
    if !db.migration_table_exists()? {
        db.create_migrations_table()?;
    }

    let ran_migrations: Vec<String> = db
        .get_migrations()?
        .iter()
        .map(|migration| migration.name.clone())
        .collect();

    let migration_root_dir = MigrationDir::new(path);

    let mut migration_dirs = migration_root_dir.get_migration_dir_names()?;

    let outstanding_migrations = migration_dirs
        .iter_mut()
        .filter(|dir_name| !ran_migrations.contains(dir_name));

    for migration in outstanding_migrations {
        let path = format!("{}/up.sql", migration);
        let up_sql = migration_root_dir.get_file_contents(&path)?;

        match db.execute_sql(&up_sql) {
            Ok(()) => migration_success(migration),
            Err(err) => {
                migration_failure(migration);
                return Err(err.into());
            }
        }

        let now = Utc::now().naive_utc();

        db.create_migration(migration, now)?;
    }

    Ok(())
}

fn migration_success(name: &str) {
    println!("    SUCCESS: {}", name);
}

fn migration_failure(name: &str) {
    println!("    FAILURE: {}", name);
}
