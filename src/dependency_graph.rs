use super::workspace::Workspace;
use std::collections::HashMap;

type DepMap = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub struct DepGraph {
    direct: DepMap,
    inversed: DepMap,
}

impl DepGraph {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        let direct = Self::build_direct_dep_graph(&workspaces);
        let inversed = Self::build_inversed_dep_graph(&workspaces);

        Self { direct, inversed }
    }

    fn build_direct_dep_graph(workspaces: &[Workspace]) -> DepMap {
        let mut map = HashMap::new();

        for ws in workspaces {
            let mut deps = vec![];

            for dep in ws.package_json.dependencies.keys() {
                deps.push(dep.to_owned());
            }

            map.insert(ws.name.clone(), deps);
        }

        map
    }

    fn build_inversed_dep_graph(workspaces: &[Workspace]) -> DepMap {
        let mut map: DepMap = HashMap::new();

        for ws in workspaces {
            for dep in ws.package_json.dependencies.keys() {
                let mut deps: Vec<String>;
                if map.contains_key(dep) {
                    deps = map.get(dep).unwrap().to_owned();
                    deps.push(ws.name.clone());
                } else {
                    deps = vec![ws.name.clone()];
                }
                map.insert(dep.clone(), deps);
            }
        }

        map
    }
}
