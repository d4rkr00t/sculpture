#!/usr/bin/env node

let cli = require("@opaline/core").default;
let pkg = require("../package.json");
let config = {
  cliName: "sculpture",
  cliVersion: pkg.version,
  cliDescription: "Use JSDoc comments to define help and parameters for a CLI.\n{cliName} will be replaced with an actual name of a CLI tool." || pkg.description,
  isSingleCommand: true,
  commands: {
    "index": {
      commandName: "index",
      meta: {"title":"Use JSDoc comments to define help and parameters for a CLI.\n{cliName} will be replaced with an actual name of a CLI tool.","description":"","usage":"sculpture inputs --param1 10 --param2 20","examples":[],"shouldPassInputs":false,"options":{}},
      load: () => {
        let command = require("./commands/index");

        if (typeof command !== "function") {
          throw new Error(`Command "index" doesn't export a function...`)
        }

        return command;
      }
    }
  }
};

cli(process.argv, config);
