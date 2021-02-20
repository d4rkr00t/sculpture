import { join } from "path";
import tsPlugin from "./plugins/typescript";
/**
 * __dirname means load native addon from current dir
 * 'sculpture-cli' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `sculpture-cli.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `sculpture-cli-[PLATFORM]`
 */
let { loadBinding } = require("@node-rs/helper");
let { Orchestrator } = loadBinding(
  join(__dirname, "..", ".."),
  "sculpture-cli",
  "sculpture-cli"
);
let plugins = [tsPlugin];

/**
 * Use JSDoc comments to define help and parameters for a CLI.
 * {cliName} will be replaced with an actual name of a CLI tool.
 *
 * @usage {cliName} inputs --param1 10 --param2 20
 */
export default async function main() {
  let start = Date.now();

  await run();

  let timing = (Date.now() - start) / 1000;
  let rounded = Math.round(timing * 100) / 100;
  console.log(`🏁  Done in ${rounded}s.`);
  process.exit(0);
}

function run() {
  return new Promise((resolve) => {
    let orchestrator = new Orchestrator({
      cwd: process.cwd(),
      onFinish() {
        resolve(undefined);
      },
      async onResolveInputs(_err: unknown, id: string, wsPath: string) {
        let promises = plugins.reduce((acc, plugin) => {
          if (!plugin.inputResolver) return acc;
          acc.push(plugin.inputResolver(wsPath));
          return acc;
        }, [] as Array<Promise<Array<String>>>);

        let files = Array.from(
          new Set((await Promise.all(promises)).flatMap((file) => file))
        );

        orchestrator.onCompleteJsTask(id, JSON.stringify(files));
      },
    });

    orchestrator.run(10);
  });
}
