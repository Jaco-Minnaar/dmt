use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use crate::{FileError, MigrationConfig};

#[derive(Debug)]
pub enum NewMigrationError {
    FileError(FileError),
}

static DEFAULT_SQL: &str = r"

    -- Write your SQL code here 
";

pub fn new_migration(name: &str, config: &MigrationConfig) -> Result<(), NewMigrationError> {
    //match new_migration_impl(opts, config) {
    //    Err(NewMigrationError::DirCreationError) => {
    //        Err("Could not create necessary directories. Please check permissions.".to_owned())
    //    }
    //    Err(NewMigrationError::FileCreationError) => {
    //        Err("Could not create necessary files. Please check permissions.".to_owned())
    //    }
    //    Err(NewMigrationError::FileWriteError) => {
    //        Err("Could not write to file. Please check permissions.".to_owned())
    //    }
    //    Ok(s) => Ok(s),
    //}
    let mut migrations_path = PathBuf::from_str(&config.migration_path)
        .map_err(|_e| NewMigrationError::FileError(FileError::Uncategorized))?;

    let now = chrono::Utc::now();
    let new_migrations_folder_name = format!("{}_{}", now.format("%Y%m%d%H%M%S"), name);

    migrations_path.push(&new_migrations_folder_name);

    fs::create_dir_all(&migrations_path)
        .map_err(|_e| NewMigrationError::FileError(FileError::DirAccessDenied))?;

    let mut up_path = migrations_path.clone();
    up_path.push("up.sql");

    let mut file = File::create(up_path)
        .map_err(|_e| NewMigrationError::FileError(FileError::FileAccessDenied))?;
    let up_sql = format!(
        "-- {} - up.sql\n{}",
        new_migrations_folder_name, DEFAULT_SQL
    );
    file.write_all(up_sql.as_bytes())
        .map_err(|_e| NewMigrationError::FileError(FileError::FileAccessDenied))?;

    let mut down_path = migrations_path.clone();
    down_path.push("down.sql");

    let mut file = File::create(down_path)
        .map_err(|_e| NewMigrationError::FileError(FileError::FileAccessDenied))?;
    let down_sql = format!(
        "-- {} - down.sql\n{}",
        new_migrations_folder_name, DEFAULT_SQL
    );
    file.write_all(down_sql.as_bytes())
        .map_err(|_e| NewMigrationError::FileError(FileError::FileAccessDenied))?;

    Ok(())
}
