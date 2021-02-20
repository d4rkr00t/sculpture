use super::file::File;
use super::package_json::PackageJson;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub files: WorkspaceFiles,
}

pub type WorkspaceFiles = HashMap<String, File>;

impl Workspace {
    pub fn new(path: String) -> Self {
        let pkg_json = PackageJson::new(&path);
        Self {
            path: Path::new(&path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            name: pkg_json.name,
            files: HashMap::new(),
        }
    }

    pub fn set_files(&mut self, files: WorkspaceFiles) {
        self.files = files;
    }

    pub fn invalidate(&self, mut resolved_inputs: Vec<String>) -> (bool, WorkspaceFiles) {
        let mut new_files: WorkspaceFiles = HashMap::new();
        let mut is_dirty = false;
        resolved_inputs.push(format!("{}/package.json", self.path));

        for file_path in resolved_inputs {
            if !self.files.contains_key(&file_path) {
                is_dirty = true;
                new_files.insert(file_path.clone(), File::new(file_path));
            } else {
                let new_file = File::new(file_path.clone());
                if self.files.get(&file_path).unwrap().hash != new_file.hash {
                    is_dirty = true;
                }
                new_files.insert(file_path, new_file);
            }
        }

        (is_dirty, new_files)
    }
}
