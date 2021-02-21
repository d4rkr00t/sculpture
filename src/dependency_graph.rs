use super::workspace::Workspace;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

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

    pub fn get_affected(&self, updated_workspaces: Vec<String>) -> Vec<String> {
        let mut affected_workspaces = HashSet::new();
        let mut queue = VecDeque::from_iter(updated_workspaces);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            affected_workspaces.insert(cur.clone());

            if !self.inversed.contains_key(&cur) {
                continue;
            }

            for ws in self.inversed.get(&cur).unwrap() {
                queue.push_back(ws.to_owned());
            }
        }

        affected_workspaces.into_iter().collect()
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
