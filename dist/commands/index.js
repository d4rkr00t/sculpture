'use strict';

var path = require('path');

const { loadBinding } = require("@node-rs/helper");

/**
 * __dirname means load native addon from current dir
 * 'sculpture-cli' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `sculpture-cli.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `sculpture-cli-[PLATFORM]`
 */
let native = loadBinding(
  path.join(__dirname, "..", ".."),
  "sculpture-cli",
  "sculpture-cli"
);

/**
 * Use JSDoc comments to define help and parameters for a CLI.
 * {cliName} will be replaced with an actual name of a CLI tool.
 *
 * @usage {cliName} inputs --param1 10 --param2 20
 */
async function main() {
  console.log(native.sync(10));
}

module.exports = main;
