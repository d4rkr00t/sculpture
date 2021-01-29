use super::file_cache::FileCache;
use super::package_json::PackageJson;
use super::workspace::Workspace;
use glob::glob;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub path: String,
    pub pkg_json: PackageJson,
    pub workspaces: HashMap<String, Workspace>,
}

impl Project {
    pub fn new(path: String) -> Project {
        let pkg_json = PackageJson::new(&format!("{}/package.json", &path));
        let workspaces_list = get_workspaces(&path, pkg_json.get_workspaces_config());

        let mut workspaces = HashMap::new();
        for ws in workspaces_list {
            if workspaces.contains_key(&ws.name) {
                panic!("Duplicage workspace with name – {}", ws.name);
            }

            workspaces.insert(ws.name.to_owned(), ws);
        }

        Project {
            path,
            pkg_json,
            workspaces,
        }
    }

    pub fn create_or_cached(cache: &FileCache, cwd: &str) -> Project {
        if cache.has("project.json") {
            println!("Project restored from cache");
            let serialized = cache.read("project.json").unwrap();
            return serde_json::from_str(&serialized).unwrap();
        }

        return Project::new(cwd.to_owned());
    }
}

fn get_workspaces(path: &str, workspaces_config: &Vec<String>) -> Vec<Workspace> {
    return workspaces_config
        .par_iter()
        .flat_map(|ws| -> Vec<Workspace> {
            let mut res: Vec<Workspace> = vec![];
            for entry in
                glob(&format!("{}/{}/package.json", path, ws)).expect("Failed to read glob pattern")
            {
                match entry {
                    Ok(p) => res.push(Workspace::new(p.into_os_string().into_string().unwrap())),
                    Err(e) => println!("{:?}", e),
                }
            }
            return res;
        })
        .collect();
}
