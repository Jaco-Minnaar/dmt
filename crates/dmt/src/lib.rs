use self::cli::{Cli, DmtCommand};
use clap::Parser;
use libdmt::{DatabaseConnection, DmtConfig, DmtError, MigrationDatabase};

mod cli;

pub fn run_dmt() -> Result<(), DmtError> {
    let cli = Cli::parse();
    let config = DmtConfig::from_file(&cli.config).map_err(DmtError::ConfigError)?;

    let mut db = MigrationDatabase::try_from(&config)?;

    handle_command(&cli.command, &mut db, &config).unwrap();

    Ok(())
}

fn handle_command(
    command: &DmtCommand,
    db: &mut impl DatabaseConnection,
    config: &DmtConfig,
) -> Result<(), DmtError> {
    match command {
        DmtCommand::New(opts) => {
            libdmt::new_migration(&opts.name, &config.migration.migration_path)?
        }
        DmtCommand::Migrate => libdmt::run_migrations(db, &config.migration.migration_path)?,
        DmtCommand::Rollback => libdmt::rollback_migrations(db, &config.migration.migration_path)?,
    };

    Ok(())
}
