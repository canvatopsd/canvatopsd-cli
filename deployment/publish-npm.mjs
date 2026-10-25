import { execFileSync } from "node:child_process";
import { archive, checkNpmArchive } from "./check-npm-archive.mjs";
import { checkRelease } from "./check-release.mjs";
import { unit, project } from "./paths.mjs";
import { join } from "node:path";

if (!process.env.npm_execpath) throw new Error("Use npm run release:npm");
const directory = await checkNpmArchive(archive);
await checkRelease(directory, { publishing: true });
const args = process.argv.slice(2);
if (args.some((arg) => arg !== "--dry-run"))
  throw new Error("Only --dry-run is supported");
// Publish the exact inspected archive; never repack source during publication.
execFileSync(
  process.execPath,
  [
    process.env.npm_execpath,
    "publish",
    archive,
    "--access",
    "public",
    "--registry=https://registry.npmjs.org/",
    "--cache",
    join(project, ".tmp/npm-cache"),
    ...args,
  ],
  { cwd: unit, stdio: "inherit" },
);
