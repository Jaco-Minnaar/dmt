use chrono::Utc;
use postgres::{Client, NoTls};

use crate::{
    config::{Database, DmtConfig},
    database::{
        create_migration, create_migrations_table, execute_sql, get_migrations,
        migration_table_exists,
    },
    io::MigrationDir,
};

enum RunMigrationsError {
    DatabaseError(String),
    FileError(String),
}

pub fn run_migrations(config: &DmtConfig) -> Result<(), String> {
    match run_migrations_impl(config) {
        Ok(_) => Ok(()),
        Err(RunMigrationsError::DatabaseError(s)) => Err(s),
        Err(RunMigrationsError::FileError(s)) => Err(s),
    }
}

fn run_migrations_impl(config: &DmtConfig) -> Result<(), RunMigrationsError> {
    match config.database {
        Database::Postgres => postgres_migrations(config),
    }
}

fn postgres_migrations(config: &DmtConfig) -> Result<(), RunMigrationsError> {
    let mut postgres_client = Client::connect(&config.connection_string, NoTls)
        .or_else(|err| Err(RunMigrationsError::DatabaseError(err.to_string())))?;

    if !migration_table_exists(&mut postgres_client)
        .or_else(|err| Err(RunMigrationsError::DatabaseError(err)))?
    {
        create_migrations_table(&mut postgres_client)
            .or_else(|err| Err(RunMigrationsError::DatabaseError(err)))?;
    }

    let ran_migrations: Vec<String> = get_migrations(&mut postgres_client)
        .or_else(|err| Err(RunMigrationsError::DatabaseError(err)))?
        .iter()
        .map(|migration| migration.name.clone())
        .collect();

    let migration_root_dir = MigrationDir::new(&config.migration_path);

    let mut migration_dirs = migration_root_dir
        .get_migration_dir_names()
        .or_else(|err| Err(RunMigrationsError::FileError(err)))?;

    let outstanding_migrations = migration_dirs
        .iter_mut()
        .filter(|dir_name| !ran_migrations.contains(&dir_name));

    for migration in outstanding_migrations {
        let path = format!("{}/up.sql", migration);
        let up_sql = migration_root_dir
            .get_file_contents(&path)
            .or_else(|err| Err(RunMigrationsError::FileError(err)))?;

        match execute_sql(&mut postgres_client, &up_sql) {
            Ok(()) => migration_success(migration),
            Err(err) => {
                migration_failure(migration);
                return Err(RunMigrationsError::DatabaseError(err.to_string()));
            }
        }

        let now = Utc::now().naive_utc();

        create_migration(&mut postgres_client, migration, now)
            .or_else(|err| Err(RunMigrationsError::FileError(err.to_string())))?;
    }

    Ok(())
}

fn migration_success(name: &str) {
    println!("    SUCCESS: {}", name);
}

fn migration_failure(name: &str) {
    println!("    FAILURE: {}", name);
}
