use chrono::Utc;
use postgres::{Client, NoTls};

use crate::{
    config::{Database, DmtConfig},
    database::{execute_sql, get_migrations, migration_table_exists, remove_migration_by_name},
    io::MigrationDir,
};

enum RollbackMigrationsError {
    DatabaseError(String),
    FileError(String),
}

pub fn rollback_migrations(config: &DmtConfig) -> Result<(), String> {
    match rollback_migrations_impl(config) {
        Ok(_) => Ok(()),
        Err(RollbackMigrationsError::DatabaseError(s)) => Err(s),
        Err(RollbackMigrationsError::FileError(s)) => Err(s),
    }
}

fn rollback_migrations_impl(config: &DmtConfig) -> Result<(), RollbackMigrationsError> {
    match config.database {
        Database::Postgres => postgres_rollback(config),
    }
}

fn postgres_rollback(config: &DmtConfig) -> Result<(), RollbackMigrationsError> {
    let mut postgres_client = Client::connect(&config.connection_string, NoTls)
        .or_else(|err| Err(RollbackMigrationsError::DatabaseError(err.to_string())))?;

    if !migration_table_exists(&mut postgres_client)
        .or_else(|err| Err(RollbackMigrationsError::DatabaseError(err)))?
    {
        println!("   No migrations have yet been run. Thus, none can be rolled back. ");
        return Ok(());
    }

    let ran_migrations_db: Vec<String> = get_migrations(&mut postgres_client)
        .or_else(|err| Err(RollbackMigrationsError::DatabaseError(err)))?
        .iter()
        .map(|migration| migration.name.clone())
        .collect();

    let migration_root_dir = MigrationDir::new(&config.migration_path);

    let mut migration_dirs = migration_root_dir
        .get_migration_dir_names()
        .or_else(|err| Err(RollbackMigrationsError::FileError(err)))?;

    let ran_migration_names = migration_dirs
        .iter_mut()
        .filter(|dir_name| ran_migrations_db.contains(&dir_name));

    let mut ran = false;
    for migration in ran_migration_names {
        ran = true;
        let path = format!("{}/down.sql", migration);
        let down_sql = migration_root_dir
            .get_file_contents(&path)
            .or_else(|err| Err(RollbackMigrationsError::FileError(err)))?;

        match execute_sql(&mut postgres_client, &down_sql) {
            Ok(()) => rollback_success(migration),
            Err(err) => {
                rollback_failure(migration);
                return Err(RollbackMigrationsError::DatabaseError(err.to_string()));
            }
        }

        remove_migration_by_name(&mut postgres_client, migration)
            .or_else(|err| Err(RollbackMigrationsError::FileError(err.to_string())))?;
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
