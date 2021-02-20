use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};

use super::js_task::JsTasksMap;
use super::project::Project;
use super::FileCache;

pub struct Runner {
    pub cwd: String,
    pub async_tasks: JsTasksMap,
    pub project: SharedProject,
    pub cache: SharedCache,
    pub on_finish: OnFinishTSFN,
    pub on_resolve: ThreadsafeFunction<Vec<String>>,
}

pub type SharedCache = Arc<RwLock<FileCache>>;
pub type SharedProject = Arc<RwLock<Project>>;
pub type OnFinishTSFN = ThreadsafeFunction<Vec<bool>>;
pub type OnResolveTSFN = ThreadsafeFunction<Vec<String>>;

impl Runner {
    pub fn new(
        cwd: String,
        cache: FileCache,
        on_finish: OnFinishTSFN,
        on_resolve: OnResolveTSFN,
    ) -> Self {
        let project = Project::create_or_cached(&cache, &cwd);
        Self {
            cwd,
            on_finish,
            on_resolve,
            cache: Arc::new(RwLock::new(cache)),
            project: Arc::new(RwLock::new(project)),
            async_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub fn run(
    shared_project: &SharedProject,
    shared_async_tasks: &JsTasksMap,
    shared_on_finish: &OnFinishTSFN,
    shared_on_resolve: &OnResolveTSFN,
    shared_cache: &SharedCache,
) {
    let shared_on_resolve_clone = shared_on_resolve.try_clone().unwrap();
    let shared_on_finish_clone = shared_on_finish.try_clone().unwrap();
    let shared_project_clone = Arc::clone(shared_project);
    let shared_async_tasks_clone = Arc::clone(shared_async_tasks);
    let shared_cache_clone = Arc::clone(shared_cache);

    thread::spawn(move || {
        let cache = shared_cache_clone
            .read()
            .expect("[runner:run] Couldn't lock read access to a cache");
        let project = shared_project_clone
            .read()
            .expect("[runner:run] Couldn't lock read access to a project");

        let (workspaces, updated) =
            project.invalidate(shared_on_resolve_clone, &shared_async_tasks_clone);
        drop(project);

        println!("Updated workspaces: {:?}", updated);

        let mut project = shared_project_clone
            .write()
            .expect("[runner:run] Couldn't lock read access to a project");

        project.workspaces = HashMap::new();
        for ws in workspaces {
            project.workspaces.insert(ws.name.to_owned(), ws);
        }

        let serialized = serde_json::to_string(&project as &Project).unwrap();
        cache.write("project.json", &serialized).unwrap();

        shared_on_finish_clone.call(Ok(vec![]), ThreadsafeFunctionCallMode::NonBlocking);
    });
}

pub fn on_complete_js_task(id: String, data: String, async_tasks: &JsTasksMap) {
    let map = async_tasks.read().expect("[orch]: RwLock");
    if map.contains_key(&id) {
        let mut state = map.get(&id).unwrap().lock().unwrap();
        if let Some(waker) = state.waker.take() {
            state.completed = true;
            state.data = Some(data);
            waker.wake();
        }
    }
}
