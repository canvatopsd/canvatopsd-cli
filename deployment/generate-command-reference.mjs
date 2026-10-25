import { cargoEnvironment } from "./cargo-environment.mjs";
import { execFileSync } from "node:child_process";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { commandReference } from "./command-reference.mjs";
import { unit, cargoTarget } from "./paths.mjs";
execFileSync(
  "cargo",
  ["build", "--locked", "--manifest-path", join(unit, "Cargo.toml")],
  {
    cwd: unit,
    env: cargoEnvironment(),
    stdio: "inherit",
  },
);
const executable = join(
  cargoTarget,
  "debug",
  process.platform === "win32" ? "canvatopsd.exe" : "canvatopsd",
);
const result = JSON.parse(
  execFileSync(executable, ["discover"], { cwd: unit, encoding: "utf8" }),
);
if (!result.ok)
  throw new Error("Cannot generate command reference from CLI discovery");
await writeFile(
  join(unit, "doc/command-reference.md"),
  await commandReference(result.data),
);
