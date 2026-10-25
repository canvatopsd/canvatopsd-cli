import { execFileSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { existsSync } from "node:fs";
export const unit = resolve(import.meta.dirname, "..");
// The CLI may be nested in the product checkout or checked out independently.
const parent = dirname(unit);
export const project = existsSync(resolve(parent, ".git")) ? parent : unit;
export const sourceRepository = unit;
export const build = resolve(project, ".tmp/build/cli");
export const cargoTarget = resolve(project, ".tmp/build/cli-cargo");
export function revision() {
  return execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: sourceRepository,
    encoding: "utf8",
  }).trim();
}
