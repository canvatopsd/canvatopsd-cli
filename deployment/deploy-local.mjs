import { execFileSync } from "node:child_process";
import { join } from "node:path";
import metadata from "../package.json" with { type: "json" };
import { archive, checkNpmArchive } from "./check-npm-archive.mjs";
import { project } from "./paths.mjs";

if (!process.env.npm_execpath) throw new Error("Use npm run deploy:local");
await checkNpmArchive(archive);
const environment = { ...process.env };
// npm --prefix cli run sets this to the source unit; global install must use the user's npm prefix.
delete environment.npm_config_prefix;
delete environment.npm_config_global_prefix;
delete environment.npm_config_local_prefix;
// Install the checked archive through npm; do not create another manual installation.
const npm = (args, options = {}) =>
  execFileSync(process.execPath, [process.env.npm_execpath, ...args], {
    cwd: project,
    env: environment,
    ...options,
  });
npm(
  [
    "install",
    "--global",
    archive,
    "--no-audit",
    "--no-fund",
    "--cache",
    join(project, ".tmp/npm-cache"),
  ],
  { stdio: "inherit" },
);
const root = npm(["root", "--global"], { encoding: "utf8" }).trim();
const entry = join(root, "@canvatopsd/cli/bin/canvatopsd.mjs");
const version = execFileSync(process.execPath, [entry, "--version"], {
  encoding: "utf8",
}).trim();
if (version !== metadata.version)
  throw new Error("Installed npm CLI version mismatch");
console.log(JSON.stringify({ environment: "local", archive, version, entry }));
