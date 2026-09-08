import { join } from "node:path";
import { root, run } from "./run-command.mjs";

try {
  // Normal verification must compare snapshots, never silently rewrite them.
  if (process.env.UPDATE_SNAPSHOTS !== undefined) {
    throw new Error("Unset UPDATE_SNAPSHOTS before running verification.");
  }
  run("Rust formatting", "cargo", ["fmt", "--all", "--", "--check"]);
  run("Rust Clippy", "cargo", ["clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"]);
  run("Rust tests and generator snapshots", "cargo", ["test", "--workspace", "--all-features"]);
  run("Rust to TypeScript integration", process.execPath, [join(root, "scripts/check-codegen.mjs")]);
  console.log("\nAll standard checks passed.");
} catch (error) {
  console.error(error.message);
  process.exitCode = 1;
}
