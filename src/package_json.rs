use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageJson {
    pub path: String,
    pub name: String,
    pub version: String,
    pub workspaces_config: Vec<String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJsonData {
    name: String,

    #[serde(default = "default_version")]
    version: String,

    #[serde(default = "default_workspaces")]
    workspaces: Vec<String>,

    #[serde(default = "default_deps")]
    dependencies: HashMap<String, String>,
}

impl PackageJson {
    pub fn new(path: &str) -> PackageJson {
        let pkg_json_data = read_pkg_json(path);
        PackageJson {
            path: path.to_owned(),
            name: pkg_json_data.name,
            version: pkg_json_data.version,
            workspaces_config: pkg_json_data.workspaces,
            dependencies: pkg_json_data.dependencies,
        }
    }

    pub fn get_workspaces_config(&self) -> &Vec<String> {
        &self.workspaces_config
    }
}

fn read_pkg_json(path: &str) -> PackageJsonData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let data: PackageJsonData =
        serde_json::from_str(&contents).expect("Cannot read package.json file");

    data
}

fn default_workspaces() -> Vec<String> {
    return vec![];
}

fn default_deps() -> HashMap<String, String> {
    HashMap::new()
}

fn default_version() -> String {
    "0.0.0".to_string()
}
