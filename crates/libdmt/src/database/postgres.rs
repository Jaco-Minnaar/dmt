use chrono::NaiveDateTime;
use postgres::{Client, NoTls, Row};

use crate::commands::Migration;
use crate::config::PostgresConfig;
use crate::ConnectionError;

use super::DatabaseConnection;

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

pub struct PostgresConnection {
    connection: Client,
}

impl PostgresConnection {
    pub fn new(config: &PostgresConfig) -> Result<Self, ConnectionError> {
        let postgres_client = Client::connect(&config.connection_string, NoTls)?;

        Ok(Self {
            connection: postgres_client,
        })
    }
}

impl DatabaseConnection for PostgresConnection {
    fn create_migrations_table(&mut self) -> Result<(), ConnectionError> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS migration (
            id SERIAL PRIMARY KEY NOT NULL,
            name VARCHAR(255) UNIQUE NOT NULL,
            time TIMESTAMP NOT NULL
        );
    "#;

        self.connection.execute(sql, &[])?;

        Ok(())
    }

    fn migration_table_exists(&mut self) -> Result<bool, ConnectionError> {
        let sql = r#"
        SELECT 1 AS "exists" FROM information_schema."tables" WHERE "table_name" = 'migration'
    "#;

        let res = self.connection.query(sql, &[])?;

        Ok(!res.is_empty())
    }

    fn get_migrations(&mut self) -> Result<Vec<Migration>, ConnectionError> {
        let sql = r#"
        SELECT * FROM migration
    "#;

        let rows = self.connection.query(sql, &[])?;

        let migrations: Vec<Migration> = rows.iter().map(|row| row.into()).collect();

        Ok(migrations)
    }

    fn create_migration(
        &mut self,
        name: &str,
        time: NaiveDateTime,
    ) -> Result<Migration, ConnectionError> {
        let sql = r#"
        INSERT INTO migration (id, name, time) VALUES (DEFAULT, $1, $2)
            RETURNING id, name, time;
    "#;

        let migration: Migration = self.connection.query_one(sql, &[&name, &time])?.into();

        Ok(migration)
    }

    fn execute_sql(&mut self, sql: &str) -> Result<(), ConnectionError> {
        let mut transaction = self.connection.transaction()?;

        transaction.batch_execute(sql)?;

        transaction.commit()?;

        Ok(())
    }

    fn remove_migration_by_id(&mut self, id: i32) -> Result<(), ConnectionError> {
        let sql = r#"
       DELETE FROM migration WHERE id = $1
    "#;

        self.connection.execute(sql, &[&id])?;

        Ok(())
    }

    fn remove_migration_by_name(&mut self, name: &str) -> Result<(), ConnectionError> {
        let sql = r#"
       DELETE FROM migration WHERE name = $1
    "#;

        self.connection.execute(sql, &[&name])?;

        Ok(())
    }
}
