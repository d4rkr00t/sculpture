mod file_cache;
mod package_json;
mod project;
mod workspace;

use file_cache::FileCache;
use neon::prelude::*;
use project::Project;
use serde_json;

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let cwd = cx.argument::<JsString>(0)?.value();
    let cache_path = format!("{}{}{}", cwd, std::path::MAIN_SEPARATOR, ".cache");
    let cache = FileCache::new(cache_path);
    let proj = Project::create_or_cached(&cache, &cwd);
    println!("{:#?}", proj);

    let serialized = serde_json::to_string(&proj).unwrap();
    cache.write("project.json", &serialized).unwrap();

    Ok(cx.number(0))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
