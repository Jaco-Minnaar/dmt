use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

use crate::MigrationError;

pub struct MigrationDir {
    path: PathBuf,
}

impl MigrationDir {
    pub fn new<P>(path: &P) -> Self
    where
        P: AsRef<Path> + ?Sized,
    {
        Self {
            path: path.as_ref().into(),
        }
    }

    pub fn get_migration_dir_names(&self) -> Result<Vec<String>, MigrationError> {
        let dir = self.dir_entries()?;

        Ok(dir
            .filter(|entry| match entry {
                Ok(entry) => {
                    if let Ok(filetype) = entry.file_type() {
                        filetype.is_dir()
                    } else {
                        false
                    }
                }
                Err(_) => false,
            })
            .map(|dir| dir.unwrap().file_name().to_string_lossy().to_string())
            .collect())
    }

    pub fn get_file_contents(&self, path: &str) -> Result<String, MigrationError> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(fs::read_to_string(&file_path)?)
    }

    fn dir_entries(&self) -> Result<ReadDir, MigrationError> {
        Ok(fs::read_dir(&self.path)?)
    }
}
