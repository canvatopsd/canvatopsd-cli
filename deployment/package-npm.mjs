import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { cp, lstat, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { checkRelease } from "./check-release.mjs";
import { archive, checkNpmArchive } from "./check-npm-archive.mjs";
import { build, project, unit } from "./paths.mjs";

const action = process.argv[2];
const generated = ["runtime", "release.json"];
const candidateDirectory = join(project, ".tmp/build/cli-npm-candidate");
const receipt = join(candidateDirectory, "checked.json");
const dryRun = process.env.npm_config_dry_run === "true";
async function existing(path) {
  return lstat(path).catch((error) => {
    if (error.code !== "ENOENT") throw error;
  });
}
async function integrity(path) {
  return (
    "sha512-" +
    createHash("sha512")
      .update(await readFile(path))
      .digest("base64")
  );
}

if (action === "prepare") {
  const archived = await existing(archive);
  if (archived && process.env.npm_command === "pack" && !dryRun)
    throw new Error(
      `Release already exists: ${archive}; run npm version patch`,
    );
  for (const name of generated)
    if (await existing(join(unit, name)))
      throw new Error(`Release path already exists: ${name}; inspect it first`);

  let prepared = false;
  try {
    let release;
    if (archived) release = await checkNpmArchive(archive);
    else {
      execFileSync(process.execPath, [join(unit, "deployment/build.mjs")], {
        stdio: "inherit",
      });
      await checkRelease(build);
      release = build;
    }
    for (const name of generated)
      await cp(join(release, name), join(unit, name), {
        recursive: true,
        force: false,
        errorOnExist: true,
      });
    await mkdir(join(unit, "releases"), { recursive: true });
    await mkdir(candidateDirectory, { recursive: true });
    const [packed] = JSON.parse(
      execFileSync(
        process.execPath,
        [
          process.env.npm_execpath,
          "pack",
          unit,
          "--ignore-scripts",
          "--dry-run=false",
          "--cache",
          join(project, ".tmp/npm-cache"),
          "--json",
          "--pack-destination",
          candidateDirectory,
        ],
        { cwd: unit, encoding: "utf8", stdio: ["ignore", "pipe", "inherit"] },
      ),
    );
    const candidate = join(candidateDirectory, packed.filename);
    await checkNpmArchive(candidate);
    if (archived && (await integrity(archive)) !== packed.integrity)
      throw new Error(
        "Source package differs from the archived version; increase the version before publishing",
      );
    await writeFile(
      receipt,
      JSON.stringify({ candidate, integrity: packed.integrity }) + "\n",
    );
    console.log(
      JSON.stringify({
        checkedNpmArchive: candidate,
        integrity: packed.integrity,
      }),
    );
    prepared = true;
  } finally {
    if (!prepared)
      for (const name of generated)
        await rm(join(unit, name), { recursive: true, force: true });
  }
} else if (action === "clean") {
  try {
    if (process.env.npm_command === "pack" && !dryRun) {
      const checked = JSON.parse(await readFile(receipt, "utf8"));
      if ((await integrity(archive)) !== checked.integrity)
        throw new Error("Final npm archive differs from the checked candidate");
      console.log(
        JSON.stringify({ archived: archive, integrity: checked.integrity }),
      );
    }
  } finally {
    for (const name of generated)
      await rm(join(unit, name), { recursive: true, force: true });
  }
} else throw new Error("Expected prepare or clean lifecycle action");
