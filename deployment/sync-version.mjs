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
const codemetaPath = join(unit, "codemeta.json");
const codemeta = JSON.parse(await readFile(codemetaPath, "utf8"));
codemeta.softwareVersion = metadata.version;
await writeFile(codemetaPath, JSON.stringify(codemeta, null, 2) + "\n");
const skillPath = join(unit, "skills/canvatopsd/SKILL.md");
const skill = await readFile(skillPath, "utf8");
await writeFile(
  skillPath,
  skill
    .replace(/^(  version: )"[^"\n]+"/m, `$1"${metadata.version}"`)
    .replace(
      /(bundled instructions describe Rust CLI )\d+\.\d+\.\d+/,
      `$1${metadata.version}`,
    ),
);
execFileSync("cargo", ["check", "--manifest-path", cargo], {
  env: cargoEnvironment(),
  stdio: "inherit",
});
