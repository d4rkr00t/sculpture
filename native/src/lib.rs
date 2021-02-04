mod file;
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

    for ws in proj.workspaces.values() {
        if ws.invalidate() {
            println!("Updated {:?}", &ws.name);
        }
    }

    let serialized = serde_json::to_string(&proj).unwrap();
    cache.write("project.json", &serialized).unwrap();

    // let args: Vec<Handle<JsNumber>> = vec![];
    // let null = cx.null();
    // let res = fun
    //     .call(&mut cx, null, args)?
    //     .downcast::<JsObject>()
    //     .unwrap();

    // let name = res.get(&mut cx, "name")?.downcast::<JsString>().unwrap();
    // println!("{:?}", name.value());

    Ok(cx.number(0))
}

// fn callOnGetChangedFiles<'a>(
//     cx: &mut FunctionContext<'a>,
// ) -> Result<Vec<String>, neon::result::Throw> {
//     let getChangedFiles = cx.argument::<JsFunction>(1)?;
//     let args: Vec<Handle<JsNumber>> = vec![];
//     let null = cx.null();

//     let res = getChangedFiles
//         .call(&mut cx, null, args)?
//         .downcast::<JsObject>()
//         .unwrap();

//     Ok(vec![])
// }

register_module!(mut cx, { cx.export_function("hello", hello) });
