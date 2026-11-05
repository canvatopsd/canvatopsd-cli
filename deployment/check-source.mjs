import { cargoEnvironment } from "./cargo-environment.mjs";
import { execFileSync } from "node:child_process";
import { readdir, readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { project, unit } from "./paths.mjs";
const provider = join(project, "web/contracts");
// Standalone public checkouts use exact provider-owned snapshots.
for (const [source, snapshot] of [
  ["task-id.rs", "src/contracts/task-id.rs"],
  ["native-runtime.rs", "src/contracts/native-runtime.rs"],
  ["native-build.rs", "deployment/native-build.rs"],
  ["ui-locale.schema.json", "src/ui-locale.schema.json"],
  ["delivery-email.schema.json", "src/delivery-email.schema.json"],
]) {
  const original = join(provider, source);
  if (
    existsSync(original) &&
    !(await readFile(original)).equals(await readFile(join(unit, snapshot)))
  )
    throw new Error(`Provider contract snapshot differs: ${snapshot}`);
}
const env = cargoEnvironment();
execFileSync("cargo-fmt", ["fmt", "--check"], {
  cwd: unit,
  env,
  stdio: "inherit",
});
execFileSync(
  "cargo-clippy",
  ["clippy", "--locked", "--all-targets", "--", "-D", "warnings"],
  { cwd: unit, env, stdio: "inherit" },
);
for (const folder of ["bin", "deployment"])
  for (const name of await readdir(join(unit, folder)))
    if (name.endsWith(".mjs"))
      execFileSync(process.execPath, ["--check", join(unit, folder, name)], {
        stdio: "inherit",
      });
execFileSync(
  process.execPath,
  [
    join(unit, "node_modules/prettier/bin/prettier.cjs"),
    "--check",
    "README.md",
    "AGENTS.md",
    "codemeta.json",
    "releases/README.md",
    "LICENSE.md",
    ".github",
    "bin",
    "deployment",
    "doc",
    "skills",
    "package.json",
  ],
  { cwd: unit, stdio: "inherit" },
);
