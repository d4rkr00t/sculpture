#![deny(clippy::all)]

mod runner;

use napi::{CallContext, Env, JsObject, JsUndefined, Property, Result};
use runner::Runner;

#[macro_use]
extern crate napi_derive;

#[js_function(1)]
fn run(ctx: CallContext) -> Result<JsUndefined> {
  let this: JsObject = ctx.this_unchecked();
  let runner: &mut Runner = ctx.env.unwrap(&this)?;
  runner.run();
  ctx.env.get_undefined()
}

#[js_function(1)]
fn complete_task(ctx: CallContext) -> Result<JsUndefined> {
  let this: JsObject = ctx.this_unchecked();
  let runner: &mut Runner = ctx.env.unwrap(&this)?;
  runner.complete_task();
  ctx.env.get_undefined()
}

#[js_function(1)]
fn runner_class_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let mut this: JsObject = ctx.this_unchecked();
  let runner = Runner::new();
  ctx.env.wrap(&mut this, runner)?;
  ctx.env.get_undefined()
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let runner = env.define_class(
    "Runner",
    runner_class_constructor,
    &[
      Property::new(&env, "run")?.with_method(run),
      Property::new(&env, "complete")?.with_method(complete_task),
    ],
  )?;
  exports.set_named_property("Runner", runner)?;
  Ok(())
}
