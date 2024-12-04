use chrono::NaiveDateTime;
use postgres::{Client, NoTls, Row};

use crate::commands::Migration;

use super::MigrationDatabase;

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

pub struct PostgresMigrationDatabase {
    connection: Client,
}

impl PostgresMigrationDatabase {
    pub fn new(connection_str: &str) -> Result<Self, ()> {
        let postgres_client = Client::connect(connection_str, NoTls).or(Err(()))?;

        Ok(Self {
            connection: postgres_client,
        })
    }
}

impl MigrationDatabase for PostgresMigrationDatabase {
    fn create_migrations_table(&mut self) -> Result<(), String> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS migration (
            id SERIAL PRIMARY KEY NOT NULL,
            name VARCHAR(255) UNIQUE NOT NULL,
            time TIMESTAMP NOT NULL
        );
    "#;

        self.connection
            .execute(sql, &[])
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn migration_table_exists(&mut self) -> Result<bool, String> {
        let sql = r#"
        SELECT 1 AS "exists" FROM information_schema."tables" WHERE "table_name" = 'migration'
    "#;

        let res = self
            .connection
            .query(sql, &[])
            .map_err(|err| err.to_string())?;

        Ok(!res.is_empty())
    }

    fn get_migrations(&mut self) -> Result<Vec<Migration>, String> {
        let sql = r#"
        SELECT * FROM migration
    "#;

        let rows = self
            .connection
            .query(sql, &[])
            .map_err(|err| err.to_string())?;

        let migrations: Vec<Migration> = rows.iter().map(|row| row.into()).collect();

        Ok(migrations)
    }

    fn create_migration(&mut self, name: &str, time: NaiveDateTime) -> Result<Migration, String> {
        let sql = r#"
        INSERT INTO migration (id, name, time) VALUES (DEFAULT, $1, $2)
            RETURNING id, name, time;
    "#;

        let migration: Migration = self
            .connection
            .query_one(sql, &[&name, &time])
            .map_err(|err| err.to_string())?
            .into();

        Ok(migration)
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), String> {
        let mut transaction = self
            .connection
            .transaction()
            .map_err(|err| err.to_string())?;

        transaction
            .batch_execute(sql)
            .map_err(|err| err.to_string())?;

        transaction.commit().map_err(|err| err.to_string())?;

        Ok(())
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), String> {
        let sql = r#"
       DELETE FROM migration WHERE id = $1
    "#;

        self.connection
            .execute(sql, &[&id])
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), String> {
        let sql = r#"
       DELETE FROM migration WHERE name = $1
    "#;

        self.connection
            .execute(sql, &[&name])
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
