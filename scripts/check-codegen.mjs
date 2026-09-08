import { copyFileSync, existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { root, run } from "./run-command.mjs";

const fixtures = join(root, "tests/e2e");
const compiler = join(root, "node_modules/typescript/bin/tsc");
let outputDir;
try {
  if (!existsSync(compiler)) throw new Error("TypeScript is missing. Run npm ci first.");
  outputDir = mkdtempSync(join(fixtures, ".generated-"));
  const output = join(outputDir, "types.ts");
  run("Generate TypeScript from Rust derives", "cargo", [
    "run", "--quiet", "--package", "gear-mesh", "--all-features", "--example", "codegen-fixture", "--", output,
  ]);
  for (const name of ["consumer.ts", "tsconfig.json"]) {
    copyFileSync(join(fixtures, name), join(outputDir, name));
  }
  run("Type-check generated output and consumer", process.execPath, [
    compiler, "--project", join(outputDir, "tsconfig.json"),
  ]);
  const expected = join(fixtures, "types.expected.ts");
  const normalize = (path) => readFileSync(path, "utf8").replaceAll("\r\n", "\n");
  if (normalize(output) !== normalize(expected)) {
    throw new Error(`Generated output differs from ${expected}. Compare with ${output}; review the specification before updating the expectation.`);
  }
  console.log("Codegen snapshot and TypeScript consumer passed.");
  rmSync(outputDir, { recursive: true });
} catch (error) {
  console.error(error.message);
  if (outputDir) console.error(`Inspection files retained in ${outputDir}`);
  process.exitCode = 1;
}
