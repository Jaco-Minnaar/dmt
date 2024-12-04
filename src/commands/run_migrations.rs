use std::path::Path;

use chrono::Utc;

use crate::{database::MigrationDatabase, io::MigrationDir};

#[derive(Debug)]
pub enum RunMigrationsError {
    DatabaseError(String),
    FileError(String),
}

pub fn run_migrations(
    db: &mut impl MigrationDatabase,
    path: &Path,
) -> Result<(), RunMigrationsError> {
    if !db
        .migration_table_exists()
        .map_err(RunMigrationsError::DatabaseError)?
    {
        db.create_migrations_table()
            .map_err(RunMigrationsError::DatabaseError)?
    }

    let ran_migrations: Vec<String> = db
        .get_migrations()
        .map_err(RunMigrationsError::DatabaseError)?
        .iter()
        .map(|migration| migration.name.clone())
        .collect();

    let migration_root_dir = MigrationDir::new(path);

    let mut migration_dirs = migration_root_dir
        .get_migration_dir_names()
        .map_err(RunMigrationsError::DatabaseError)?;

    let outstanding_migrations = migration_dirs
        .iter_mut()
        .filter(|dir_name| !ran_migrations.contains(dir_name));

    for migration in outstanding_migrations {
        let path = format!("{}/up.sql", migration);
        let up_sql = migration_root_dir
            .get_file_contents(&path)
            .map_err(RunMigrationsError::DatabaseError)?;

        match db.execute_sql(&up_sql) {
            Ok(()) => migration_success(migration),
            Err(err) => {
                migration_failure(migration);
                return Err(RunMigrationsError::DatabaseError(err.to_string()));
            }
        }

        let now = Utc::now().naive_utc();

        db.create_migration(migration, now)
            .map_err(RunMigrationsError::DatabaseError)?;
    }

    Ok(())
}

fn migration_success(name: &str) {
    println!("    SUCCESS: {}", name);
}

fn migration_failure(name: &str) {
    println!("    FAILURE: {}", name);
}
