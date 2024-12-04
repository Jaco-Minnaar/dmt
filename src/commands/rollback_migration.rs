use std::path::Path;

use crate::{database::MigrationDatabase, io::MigrationDir};

#[derive(Debug)]
pub enum RollbackMigrationsError {
    DatabaseError(String),
    FileError(String),
}

pub fn rollback_migrations(
    db: &mut impl MigrationDatabase,
    path: &Path,
) -> Result<(), RollbackMigrationsError> {
    if !db
        .migration_table_exists()
        .map_err(RollbackMigrationsError::DatabaseError)?
    {
        println!("   No migrations have yet been run. Thus, none can be rolled back. ");
        return Ok(());
    }

    let ran_migrations_db: Vec<String> = db
        .get_migrations()
        .map_err(RollbackMigrationsError::DatabaseError)?
        .iter()
        .map(|migration| migration.name.clone())
        .collect();

    let migration_root_dir = MigrationDir::new(path);

    let mut migration_dirs = migration_root_dir
        .get_migration_dir_names()
        .map_err(RollbackMigrationsError::DatabaseError)?;

    let ran_migration_names = migration_dirs
        .iter_mut()
        .filter(|dir_name| ran_migrations_db.contains(dir_name));

    let mut ran = false;
    for migration in ran_migration_names {
        ran = true;
        let path = format!("{}/down.sql", migration);
        let down_sql = migration_root_dir
            .get_file_contents(&path)
            .map_err(RollbackMigrationsError::DatabaseError)?;

        match db.execute_sql(&down_sql) {
            Ok(()) => rollback_success(migration),
            Err(err) => {
                rollback_failure(migration);
                return Err(RollbackMigrationsError::DatabaseError(err.to_string()));
            }
        }

        db.remove_migration_by_name(migration)
            .map_err(RollbackMigrationsError::DatabaseError)?;
    }

    if !ran {
        println!("   No migrations have yet been run. Thus, none can be rolled back. ");
    }

    Ok(())
}

fn rollback_success(name: &str) {
    println!("    SUCCESS: {}", name);
}

fn rollback_failure(name: &str) {
    println!("    FAILURE: {}", name);
}
