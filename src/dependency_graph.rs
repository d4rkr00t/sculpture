use super::workspace::Workspace;
use semver::{Version, VersionReq};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type DepMap = HashMap<String, (String, Vec<(String, String)>)>;

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

    pub fn get_affected(&self, updated_workspaces: Vec<String>) -> Result<Vec<String>, String> {
        let mut affected_workspaces = HashSet::new();
        let mut queue = VecDeque::from_iter(updated_workspaces);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            affected_workspaces.insert(cur.clone());

            if !self.inversed.contains_key(&cur) {
                continue;
            }

            for (_, ws) in &self.inversed.get(&cur).unwrap().1 {
                queue.push_back(ws.to_owned());
            }
        }

        self.top_sort(affected_workspaces.into_iter().collect())
    }

    fn top_sort(&self, workspaces: HashSet<String>) -> Result<Vec<String>, String> {
        fn dfs(
            cur: &str,
            graph: &DepMap,
            workspaces: &HashSet<String>,
            visited: &mut HashSet<String>,
            path: &mut HashSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), String> {
            if path.contains(cur) {
                let mut path_vec = path.iter().collect::<Vec<&String>>();
                let cur_copy = cur.to_owned();
                path_vec.push(&cur_copy);
                path_vec.reverse();
                return Err(format!("Cycle detected: {:?}", path_vec));
            }

            path.insert(cur.to_owned());

            if !graph.contains_key(cur) {
                visited.insert(cur.to_owned());
                sorted.push(cur.to_owned());
                path.remove(cur);
                return Ok(());
            }

            for (_, dep) in &graph.get(cur).unwrap().1 {
                if workspaces.contains(dep) && !visited.contains(dep) {
                    match dfs(dep, graph, workspaces, visited, path, sorted) {
                        Ok(_) => continue,
                        Err(e) => return Err(e),
                    }
                }
            }

            visited.insert(cur.to_owned());
            path.remove(cur);
            sorted.push(cur.to_owned());

            Ok(())
        }

        let mut sorted_workspaces: Vec<String> = vec![];
        let mut visited: HashSet<String> = HashSet::new();
        let mut path: HashSet<String> = HashSet::new();

        for ws in &workspaces {
            if visited.contains(ws) {
                continue;
            }

            match dfs(
                ws,
                &self.direct,
                &workspaces,
                &mut visited,
                &mut path,
                &mut sorted_workspaces,
            ) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(sorted_workspaces)
    }

    pub fn validate(&self) -> bool {
        self.validate_versions() && self.validate_cycles()
    }

    fn validate_versions(&self) -> bool {
        let mut dep_to_version = HashMap::new();

        fn get_clean_version(version: &str) -> String {
            version.replace("^", "").replace("~", "")
        }
        for (ws_name, (ws_version, _)) in &self.direct {
            dep_to_version.insert(ws_name.clone(), get_clean_version(ws_version));
        }

        for (ws_name, (_, deps)) in &self.direct {
            for (version, name) in deps {
                if dep_to_version.contains_key(name) {
                    let version_req = VersionReq::parse(version).unwrap();
                    let version_parsed = Version::parse(dep_to_version.get(name).unwrap()).unwrap();
                    if !version_req.matches(&version_parsed) {
                        println!("Package \"{}\" depends on \"{}@{}\", but there is already \"{}@{}\". Only one version is allowed.", ws_name, name, version, name, dep_to_version.get(name).unwrap());
                        return false;
                    }
                } else {
                    dep_to_version.insert(name.clone(), get_clean_version(version));
                }
            }
        }

        true
    }

    fn validate_cycles(&self) -> bool {
        let workspaces: HashSet<String> = self.direct.keys().cloned().collect();
        match self.top_sort(workspaces) {
            Ok(_) => true,
            Err(e) => {
                println!("{}", e);
                false
            }
        }
    }

    fn build_direct_dep_graph(workspaces: &[Workspace]) -> DepMap {
        let mut map = HashMap::new();

        for ws in workspaces {
            let mut deps = vec![];

            for (dep, version) in &ws.package_json.dependencies {
                deps.push((version.to_owned(), dep.to_owned()));
            }

            map.insert(ws.name.clone(), (ws.package_json.version.clone(), deps));
        }

        map
    }

    fn build_inversed_dep_graph(workspaces: &[Workspace]) -> DepMap {
        let mut map: DepMap = HashMap::new();

        for ws in workspaces {
            for (dep, version) in &ws.package_json.dependencies {
                let mut deps: Vec<(String, String)>;
                if map.contains_key(dep) {
                    deps = map.get(dep).unwrap().1.to_owned();
                    deps.push((ws.package_json.version.clone(), ws.name.clone()));
                } else {
                    deps = vec![(ws.package_json.version.clone(), ws.name.clone())];
                }
                map.insert(dep.clone(), (version.clone(), deps));
            }
        }

        map
    }
}
