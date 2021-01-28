const addon = require("../../native");

/**
 * Use JSDoc comments to define help and parameters for a CLI.
 * {cliName} will be replaced with an actual name of a CLI tool.
 *
 * @usage {cliName} inputs --param1 10 --param2 20
 */
export default async function main() {
  console.log(addon.hello());
}
