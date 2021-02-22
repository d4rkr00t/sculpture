use super::file::File;
use super::package_json::PackageJson;
use async_std::task;
// use futures::future::join_all;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub files: WorkspaceFiles,
    pub package_json: PackageJson,
}

pub type WorkspaceFiles = HashMap<String, File>;

impl Workspace {
    pub fn new(path: String) -> Self {
        let pkg_json = PackageJson::new(&path);
        Self {
            name: pkg_json.name.clone(),
            path: Path::new(&path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            files: HashMap::new(),
            package_json: pkg_json,
        }
    }

    pub fn set_files(&mut self, files: WorkspaceFiles) {
        self.files = files;
    }

    pub fn invalidate(&self, mut resolved_inputs: Vec<String>) -> (bool, WorkspaceFiles) {
        let mut new_files: WorkspaceFiles = HashMap::new();
        let mut is_dirty = false;
        let mut futures_list = FuturesUnordered::new();
        resolved_inputs.push(format!("{}/package.json", self.path));

        for file_path in resolved_inputs {
            let fut = async move {
                if !self.files.contains_key(&file_path) {
                    let new_file = File::new(file_path.clone()).await;
                    return (true, new_file);
                } else {
                    return self.files.get(&file_path).unwrap().invalidate().await;
                }
            };
            futures_list.push(fut);
        }

        task::block_on(async {
            while let Some((dirty, new_file)) = futures_list.next().await {
                if !is_dirty {
                    is_dirty = dirty;
                }
                new_files.insert(new_file.path.clone(), new_file);
            }
        });

        (is_dirty, new_files)
    }
}
