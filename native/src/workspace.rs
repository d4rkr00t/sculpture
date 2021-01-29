use super::package_json::PackageJson;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub dependencies: HashMap<String, String>,
    pub files: Vec<String>,
}

impl Workspace {
    pub fn new(path: String) -> Workspace {
        let pkg_json = PackageJson::new(&path);
        Workspace {
            path: Path::new(&path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            name: pkg_json.name,
            dependencies: pkg_json.dependencies,
            files: vec![path],
        }
    }
}
