use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct FileCache {
    path: String,
}

impl FileCache {
    pub fn new(path: String) -> FileCache {
        FileCache { path }
    }

    pub fn read(&self, file_name: &str) -> std::io::Result<String> {
        let mut file = File::open(self.get_cache_file_path(file_name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write(&self, file_name: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(self.get_cache_file_path(file_name))?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn has(&self, file_name: &str) -> bool {
        Path::new(&self.get_cache_file_path(file_name)).exists()
    }

    fn get_cache_file_path(&self, file_name: &str) -> String {
        return format!("{}{}{}", self.path, std::path::MAIN_SEPARATOR, file_name);
    }
}
