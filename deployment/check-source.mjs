import { cargoEnvironment } from "./cargo-environment.mjs";
import { execFileSync } from "node:child_process";
import { readdir } from "node:fs/promises";
import { join } from "node:path";
import { unit } from "./paths.mjs";
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
