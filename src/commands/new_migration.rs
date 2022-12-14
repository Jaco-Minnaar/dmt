use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

use crate::{cli::NewMigrationArgs, config::DmtConfig, DmtError, FileError};

pub enum NewMigrationError {
    DirCreationError,
    FileCreationError,
    FileWriteError,
}

pub fn new_migration(opts: &NewMigrationArgs, config: &DmtConfig) -> Result<(), String> {
    match new_migration_impl(opts, config) {
        Err(NewMigrationError::DirCreationError) => {
            Err("Could not create necessary directories. Please check permissions.".to_owned())
        }
        Err(NewMigrationError::FileCreationError) => {
            Err("Could not create necessary files. Please check permissions.".to_owned())
        }
        Err(NewMigrationError::FileWriteError) => {
            Err("Could not write to file. Please check permissions.".to_owned())
        }
        Ok(s) => Ok(s),
    }
}

static DEFAULT_SQL: &str = r"

    -- Write your SQL code here 
";

pub fn new_migration_impl(
    opts: &NewMigrationArgs,
    config: &DmtConfig,
) -> Result<(), NewMigrationError> {
    let mut migrations_path = config.migration_path.clone();

    let now = chrono::Utc::now();
    let new_migrations_folder_name = format!("{}_{}", now.format("%Y%m%d%H%M%S"), opts.name);

    migrations_path.push(&new_migrations_folder_name);

    fs::create_dir_all(&migrations_path).or(Err(NewMigrationError::DirCreationError))?;

    let mut up_path = migrations_path.clone();
    up_path.push("up.sql");

    let mut file = File::create(up_path).or(Err(NewMigrationError::FileCreationError))?;
    let up_sql = format!(
        "-- {} - up.sql\n{}",
        new_migrations_folder_name, DEFAULT_SQL
    );
    file.write_all(up_sql.as_bytes())
        .or(Err(NewMigrationError::FileWriteError))?;

    let mut down_path = migrations_path.clone();
    down_path.push("down.sql");

    let mut file = File::create(down_path).or(Err(NewMigrationError::FileCreationError))?;
    let down_sql = format!(
        "-- {} - down.sql\n{}",
        new_migrations_folder_name, DEFAULT_SQL
    );
    file.write_all(down_sql.as_bytes())
        .or(Err(NewMigrationError::FileWriteError))?;

    Ok(())
}
