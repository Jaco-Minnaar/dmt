use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

pub struct MigrationDir {
    path: PathBuf,
}

impl MigrationDir {
    pub fn new(path: &Path) -> Self {
        Self { path: path.into() }
    }

    pub fn get_migration_dir_names(&self) -> Result<Vec<String>, String> {
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

    pub fn get_file_contents(&self, path: &str) -> Result<String, String> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        fs::read_to_string(&file_path).map_err(|err| err.to_string())
    }

    fn dir_entries(&self) -> Result<ReadDir, String> {
        fs::read_dir(&self.path).map_err(|err| err.to_string())
    }
}
