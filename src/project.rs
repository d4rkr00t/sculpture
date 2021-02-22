use super::file_cache::FileCache;
use super::js_task::{JsTask, JsTasksMap};
use super::package_json::PackageJson;
use super::workspace::Workspace;
use async_std::task;
use futures::stream::{FuturesUnordered, StreamExt};
use glob::glob;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    path: String,
    pkg_json: PackageJson,
    pub workspaces: HashMap<String, Workspace>,
}

impl Project {
    pub fn new(path: String) -> Self {
        let pkg_json = PackageJson::new(&format!("{}/package.json", &path));
        let workspaces_list = get_workspaces(&path, pkg_json.get_workspaces_config());
        let mut workspaces = HashMap::new();
        for ws in workspaces_list {
            if workspaces.contains_key(&ws.name) {
                panic!("Duplicate workspace with name – {}", ws.name);
            }

            workspaces.insert(ws.name.to_owned(), ws);
        }

        Self {
            path,
            pkg_json,
            workspaces,
        }
    }

    pub fn create_or_cached(cache: &FileCache, cwd: &str) -> Project {
        if cache.has("project.json") {
            println!("Project restored from cache");
            println!();
            println!("--------------");
            println!();
            let serialized = cache.read("project.json").unwrap();
            return serde_json::from_str(&serialized).unwrap();
        }

        Project::new(cwd.to_owned())
    }

    pub fn invalidate(
        &self,
        on_resolve: ThreadsafeFunction<Vec<String>>,
        async_tasks: &JsTasksMap,
    ) -> (Vec<Workspace>, Vec<String>) {
        print!("Creating a list of workspaces... ");
        let ws_start = Instant::now();
        let workspaces_list = get_workspaces(&self.path, self.pkg_json.get_workspaces_config());
        let mut future_list = FuturesUnordered::new();
        println!("[{} ms]", ws_start.elapsed().as_millis());

        let invalidate_fut_list_start = Instant::now();
        print!("Creating a list of workspace invalidate futures... ");
        for cur_ws in workspaces_list {
            let ws = if self.workspaces.contains_key(&cur_ws.name) {
                self.workspaces.get(&cur_ws.name).unwrap().clone()
            } else {
                cur_ws.clone()
            };

            let mut map = async_tasks.write().expect("RwLock poisoned");
            let task = JsTask::new(format!("{}:{}", ws.name, "resolve_inputs"));
            let state_clone = task.state.clone();
            let on_resolve_clone = on_resolve.try_clone().unwrap();
            map.insert(task.id.clone(), task.state.clone());
            drop(map);

            let fut = async move {
                on_resolve_clone.call(
                    Ok(vec![task.id.clone(), ws.path.clone()]),
                    ThreadsafeFunctionCallMode::NonBlocking,
                );

                task.await;
                let state = state_clone.lock().unwrap();
                if let Some(data) = &state.data {
                    let files: Vec<String> = serde_json::from_str(data).unwrap();
                    let (is_dirty, new_files) = ws.invalidate(files);
                    if is_dirty {
                        let mut new_ws = ws.clone();
                        new_ws.set_files(new_files);
                        return Some((true, new_ws));
                    }
                    return Some((false, ws));
                }

                return None;
            };

            future_list.push(fut);
        }
        println!("[{} ms]", invalidate_fut_list_start.elapsed().as_millis());

        let mut result_workspaces: Vec<Workspace> = vec![];
        let mut updated_workspaces: Vec<String> = vec![];

        task::block_on(async {
            while let Some(workspaces) = future_list.next().await {
                match workspaces {
                    Some((true, ws)) => {
                        updated_workspaces.push(ws.name.to_owned());
                        result_workspaces.push(ws);
                    }
                    Some((false, ws)) => {
                        result_workspaces.push(ws);
                    }
                    None => {}
                }
            }
        });

        (result_workspaces, updated_workspaces)
    }
}

fn get_workspaces(path: &str, workspaces_config: &[String]) -> Vec<Workspace> {
    let mut result_workspaces: Vec<Workspace> = vec![];

    for ws in workspaces_config {
        for entry in
            glob(&format!("{}/{}/package.json", path, ws)).expect("Failed to read glob pattern")
        {
            match entry {
                Ok(p) => result_workspaces
                    .push(Workspace::new(p.into_os_string().into_string().unwrap())),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    result_workspaces
}
