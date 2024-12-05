use self::cli::{Cli, DmtCommand};
use clap::Parser;
use dmt::{
    Database, DatabaseConnection, DmtConfig, DmtError, FromDatabaseConfig, MigrationDatabase,
};

mod cli;

pub fn run_dmt() -> bool {
    let cli = Cli::parse();
    let config = if let Ok(config) = DmtConfig::from_path(cli.config.clone()) {
        config
    } else {
        eprintln!(
            "Could not open config file: {}",
            cli.config.to_string_lossy()
        );
        return false;
    };

    let Ok(mut db) = (match config.connection.database {
        Some(Database::Postgres) => {
            if let Some(config) = &config.connection.postgres {
                MigrationDatabase::from_database_config(config)
            } else {
                eprintln!("No postgres config found");
                return false;
            }
        }
        Some(Database::Turso) => {
            if let Some(config) = &config.connection.turso {
                MigrationDatabase::from_database_config(config)
            } else {
                eprintln!("No postgres config found");
                return false;
            }
        }
        None => {
            eprintln!("Database type not specified in config");
            return false;
        }
    }) else {
        eprintln!("Could not establish connection to database.",);
        return false;
    };

    handle_command(&cli.command, &mut db, &config).unwrap();

    true
}

fn handle_command(
    command: &DmtCommand,
    db: &mut impl DatabaseConnection,
    config: &DmtConfig,
) -> Result<(), DmtError> {
    match command {
        DmtCommand::New(opts) => {
            dmt::new_migration(&opts.name, &config.migration).map_err(DmtError::NewMigrationError)
        }
        DmtCommand::Migrate => dmt::run_migrations(db, &config.migration.migration_path)
            .map_err(DmtError::RunMigrationError),
        DmtCommand::Rollback => dmt::rollback_migrations(db, &config.migration.migration_path)
            .map_err(DmtError::RollbackMigrationsError),
    }
}
