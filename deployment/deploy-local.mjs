import { execFileSync } from "node:child_process";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";
import metadata from "../package.json" with { type: "json" };
import { nativeExecutable } from "../bin/native-executable.mjs";
import { checkNpmArchive } from "./check-npm-archive.mjs";
import { checkRelease } from "./check-release.mjs";
import { localBuild, project, unit } from "./paths.mjs";
import { hostTarget } from "./targets.mjs";

if (!process.env.npm_execpath) throw new Error("Use npm run deploy:local");
const environment = { ...process.env };
// Global installation uses the user's npm prefix, not npm --prefix cli.
delete environment.npm_config_prefix;
delete environment.npm_config_global_prefix;
delete environment.npm_config_local_prefix;
const npm = (args, options = {}) =>
  execFileSync(process.execPath, [process.env.npm_execpath, ...args], {
    cwd: project,
    env: environment,
    ...options,
  });

execFileSync(process.execPath, [join(unit, "deployment/build.mjs"), "local"], {
  cwd: unit,
  env: { ...environment, CANVATOPSD_BUILD_TARGETS: hostTarget },
  stdio: "inherit",
});
await checkRelease(localBuild, { environment: "local" });
const candidates = join(project, ".tmp/build/cli-local-npm");
await mkdir(candidates, { recursive: true });
const [packed] = JSON.parse(
  npm(
    [
      "pack",
      localBuild,
      "--ignore-scripts",
      "--json",
      "--pack-destination",
      candidates,
      "--cache",
      join(project, ".tmp/npm-cache"),
    ],
    { encoding: "utf8" },
  ),
);
const archive = join(candidates, packed.filename);
await checkNpmArchive(archive, { environment: "local" });
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
const installed = join(root, "@canvatopsd/cli");
const entry = join(installed, "bin/canvatopsd.mjs");
const binary = join(
  installed,
  "runtime",
  hostTarget,
  nativeExecutable(metadata.version, "local", hostTarget),
);
const identity = JSON.parse(
  execFileSync(binary, ["installation-info"], { encoding: "utf8" }),
);
if (
  identity.version !== metadata.version ||
  identity.environment !== "local" ||
  identity.embeddedConfiguration !== false
)
  throw new Error("Installed local CLI identity mismatch");
const localOrigin =
  process.env.CANVATOPSD_LOCAL_WEB_ORIGIN ?? "http://localhost:8795";
execFileSync(
  process.execPath,
  [
    entry,
    "--origin",
    localOrigin,
    "init",
    new URL("/api", localOrigin).href,
    "--internal",
  ],
  { stdio: "inherit" },
);
console.log(
  JSON.stringify({
    environment: "local",
    archive,
    version: metadata.version,
    entry,
    binary,
    stateRoot: identity.root,
  }),
);
