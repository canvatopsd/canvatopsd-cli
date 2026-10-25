import { cargoEnvironment } from "./cargo-environment.mjs";
import { readFile, writeFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { unit } from "./paths.mjs";
const metadata = JSON.parse(await readFile(join(unit, "package.json"), "utf8"));
const cargo = join(unit, "Cargo.toml");
const source = await readFile(cargo, "utf8");
await writeFile(
  cargo,
  source.replace(/^version = "[^"]+"/m, `version = "${metadata.version}"`),
);
execFileSync("cargo", ["check", "--manifest-path", cargo], {
  env: cargoEnvironment(),
  stdio: "inherit",
});
