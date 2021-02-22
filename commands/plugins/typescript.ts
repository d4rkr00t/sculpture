import { promises } from "fs";
import { join } from "path";
import fg from "fast-glob";
import ts from "typescript";

async function exist(file_path: string) {
  try {
    let stats = await promises.stat(file_path);
    return stats.isFile;
  } catch {
    return false;
  }
}

const tsPlugin = {
  async inputResolver(ws: string): Promise<string[]> {
    let tsconfigPath = join(ws, "tsconfig.json");
    if (await !exist(tsconfigPath)) {
      return [];
    }

    try {
      let file = await promises.readFile(tsconfigPath, "utf8");
      let tsconfig = ts.parseConfigFileTextToJson("tsconfig.json", file).config;
      let glob_inputs = await fg(tsconfig.include ?? [], {
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
