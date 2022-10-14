use postgres::{Client, NoTls};

use crate::config::{Database, DmtConfig};

use super::Migration;

enum RunMigrationsError {
    DatabaseError(String),
}

pub fn run_migrations(config: &DmtConfig) -> Result<(), String> {
    match run_migrations_impl(config) {
        Ok(_) => Ok(()),
        Err(RunMigrationsError::DatabaseError(s)) => Err(s),
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

    let sql = r"
        CREATE TABLE IF NOT EXISTS migrations (
            id SERIAL PRIMARY KEY NOT NULL,
            name VARCHAR(255) UNIQUE NOT NULL,
            time TIMESTAMP NOT NULL
        );
    ";

    postgres_client
        .execute(sql, &[])
        .or_else(|err| Err(RunMigrationsError::DatabaseError(err.to_string())))?;

    let sql = r"
        SELECT * FROM migrations;
    ";

    let rows = postgres_client
        .query(sql, &[])
        .or_else(|err| Err(RunMigrationsError::DatabaseError(err.to_string())))?;

    let migrations = rows.iter().map(|row| Migration {
        id: row.get(0),
        name: row.get(1),
        time: row.get(2),
    });

    Ok(())
}
