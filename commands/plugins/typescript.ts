import { statSync } from "fs";
import { join } from "path";
import fg from "fast-glob";

function exist(file_path: string) {
  try {
    let stats = statSync(file_path);
    return stats.isFile;
  } catch {
    return false;
  }
}

const tsPlugin = {
  inputResolver(ws: string): string[] {
    let tsconfigPath = join(ws, "tsconfig.json");
    if (!exist(tsconfigPath)) {
      return [];
    }

    try {
      let tsconfig = require(tsconfigPath);
      let glob_inputs = fg.sync(tsconfig.include ?? [], {
        ignore: [...(tsconfig.exclude ?? []), "node_modules"],
        dot: true,
        cwd: ws,
      });

      return ["tsconfig.json", ...glob_inputs].map((file_path) =>
        join(ws, file_path)
      );
    } catch (e) {
      return [];
    }
  },
};

export default tsPlugin;
