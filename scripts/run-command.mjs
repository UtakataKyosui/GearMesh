import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

export const root = fileURLToPath(new URL("../", import.meta.url));

export function run(label, command, args) {
  console.log(`\n[${label}]`);
  const result = spawnSync(command, args, { cwd: root, stdio: "inherit" });
  if (result.error) throw new Error(`${label}: ${result.error.message}`);
  if (result.status !== 0) {
    throw new Error(`${label} failed (${result.signal ?? `exit ${result.status}`})`);
  }
}
