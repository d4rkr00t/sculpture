#![deny(clippy::all)]

mod file;
mod file_cache;
mod js_task;
mod package_json;
mod project;
mod runner;
mod workspace;

use file_cache::FileCache;
use napi::{
  threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction},
  CallContext, Env, JsFunction, JsObject, JsString, JsUndefined, Property, Result,
};
use runner::{on_complete_js_task, run, Runner};

#[macro_use]
extern crate napi_derive;

#[js_function(1)]
fn run_js_interface(ctx: CallContext) -> Result<JsUndefined> {
  let this: JsObject = ctx.this_unchecked();
  let runner: &mut Runner = ctx.env.unwrap(&this)?;

  run(
    &runner.project,
    &runner.async_tasks,
    &runner.on_finish,
    &runner.on_resolve,
    &runner.cache,
  );

  ctx.env.get_undefined()
}

#[js_function(2)]
fn on_complete_js_task_js_interface(ctx: CallContext) -> Result<JsUndefined> {
  let this: JsObject = ctx.this_unchecked();
  let id = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let data = ctx.get::<JsString>(1)?.into_utf8()?.as_str()?.to_owned();
  let runner: &mut Runner = ctx.env.unwrap(&this)?;

  on_complete_js_task(id, data, &runner.async_tasks);

  ctx.env.get_undefined()
}

#[js_function(1)]
fn runner_class_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let mut this: JsObject = ctx.this_unchecked();

  let params = ctx.get::<JsObject>(0)?;
  let cwd = params
    .get_named_property::<JsString>("cwd")?
    .into_utf8()?
    .as_str()?
    .to_owned();

  let on_finish = create_on_finish(&ctx, params.get_named_property::<JsFunction>("onFinish")?)?;
  let on_resolve = create_on_resolve(
    &ctx,
    params.get_named_property::<JsFunction>("onResolveInputs")?,
  )?;
  let cache_path = format!("{}{}{}", cwd, std::path::MAIN_SEPARATOR, ".cache");
  let cache = FileCache::new(cache_path);

  let runner = Runner::new(cwd, cache, on_finish, on_resolve);
  ctx.env.wrap(&mut this, runner)?;
  ctx.env.get_undefined()
}

fn create_on_finish(
  ctx: &CallContext,
  on_finish_cb: JsFunction,
) -> Result<ThreadsafeFunction<Vec<bool>>> {
  Ok(ctx.env.create_threadsafe_function(
    &on_finish_cb,
    0,
    |ctx: ThreadSafeCallContext<Vec<bool>>| {
      ctx
        .value
        .iter()
        .map(|_| ctx.env.get_undefined())
        .collect::<Result<Vec<JsUndefined>>>()
    },
  )?)
}

fn create_on_resolve(
  ctx: &CallContext,
  on_finish_cb: JsFunction,
) -> Result<ThreadsafeFunction<Vec<String>>> {
  Ok(ctx.env.create_threadsafe_function(
    &on_finish_cb,
    0,
    |ctx: ThreadSafeCallContext<Vec<String>>| {
      ctx
        .value
        .iter()
        .map(|v| ctx.env.create_string(&*v))
        .collect::<Result<Vec<JsString>>>()
    },
  )?)
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let runner = env.define_class(
    "Runner",
    runner_class_constructor,
    &[
      Property::new(&env, "run")?.with_method(run_js_interface),
      Property::new(&env, "onCompleteJsTask")?.with_method(on_complete_js_task_js_interface),
    ],
  )?;
  exports.set_named_property("Orchestrator", runner)?;
  Ok(())
}
