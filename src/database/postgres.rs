use chrono::NaiveDateTime;
use postgres::{Client, Row};

use crate::commands::Migration;

impl From<Row> for Migration {
    fn from(row: Row) -> Self {
        Migration {
            id: row.get(0),
            name: row.get(1),
            time: row.get(2),
        }
    }
}

impl From<&Row> for Migration {
    fn from(row: &Row) -> Self {
        Migration {
            id: row.get(0),
            name: row.get(1),
            time: row.get(2),
        }
    }
}

pub fn create_migrations_table(connnection: &mut Client) -> Result<(), String> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS migration (
            id SERIAL PRIMARY KEY NOT NULL,
            name VARCHAR(255) UNIQUE NOT NULL,
            time TIMESTAMP NOT NULL
        );
    "#;

    connnection
        .execute(sql, &[])
        .or_else(|err| Err(err.to_string()))?;

    Ok(())
}

pub fn migration_table_exists(connection: &mut Client) -> Result<bool, String> {
    let sql = r#"
        SELECT 1 AS "exists" FROM information_schema."tables" WHERE "table_name" = 'migration'
    "#;

    let res = connection
        .query(sql, &[])
        .or_else(|err| Err(err.to_string()))?;

    Ok(res.len() > 0)
}

pub fn get_migrations(connection: &mut Client) -> Result<Vec<Migration>, String> {
    let sql = r#"
        SELECT * FROM migration
    "#;

    let rows = connection
        .query(sql, &[])
        .or_else(|err| Err(err.to_string()))?;

    let migrations: Vec<Migration> = rows.iter().map(|row| row.into()).collect();

    Ok(migrations)
}

pub fn create_migration(
    connection: &mut Client,
    name: &str,
    time: NaiveDateTime,
) -> Result<Migration, String> {
    let sql = r#"
        INSERT INTO migration (id, name, time) VALUES (DEFAULT, $1, $2)
            RETURNING id, name, time;
    "#;

    let migration: Migration = connection
        .query_one(sql, &[&name, &time])
        .or_else(|err| Err(err.to_string()))?
        .into();

    Ok(migration)
}

pub fn execute_sql(connection: &mut Client, sql: &str) -> Result<(), String> {
    let mut transaction = connection
        .transaction()
        .or_else(|err| Err(err.to_string()))?;

    transaction
        .batch_execute(sql)
        .or_else(|err| Err(err.to_string()))?;

    transaction.commit().or_else(|err| Err(err.to_string()))?;

    Ok(())
}

pub fn remove_migration_by_id(connection: &mut Client, id: i32) -> Result<(), String> {
    let sql = r#"
       DELETE FROM migration WHERE id = $1
    "#;

    connection
        .execute(sql, &[&id])
        .or_else(|err| Err(err.to_string()))?;

    Ok(())
}

pub fn remove_migration_by_name(connection: &mut Client, name: &str) -> Result<(), String> {
    let sql = r#"
       DELETE FROM migration WHERE name = $1
    "#;

    connection
        .execute(sql, &[&name])
        .or_else(|err| Err(err.to_string()))?;

    Ok(())
}
