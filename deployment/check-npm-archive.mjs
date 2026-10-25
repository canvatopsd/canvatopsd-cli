import { mkdir, mkdtemp } from "node:fs/promises";
import { join, resolve } from "node:path";
import { t, x } from "tar";
import metadata from "../package.json" with { type: "json" };
import { checkRelease } from "./check-release.mjs";
import { project, unit } from "./paths.mjs";

export const archive = join(
  unit,
  "releases",
  `canvatopsd-cli-${metadata.version}.tgz`,
);

export async function checkNpmArchive(file = archive) {
  const entries = [];
  await t({
    file,
    strict: true,
    onReadEntry: (entry) => entries.push(entry.path),
  });
  for (const name of entries)
    if (!name.startsWith("package/") || name.split("/").includes(".."))
      throw new Error(`Unexpected npm archive path: ${name}`);
  if (entries.some((name) => name.startsWith("package/releases/")))
    throw new Error("npm archive contains release history");
  const checks = join(project, ".tmp/artifacts/cli-archive-check");
  await mkdir(checks, { recursive: true });
  const directory = await mkdtemp(join(checks, `${metadata.version}-`));
  await x({ file, cwd: directory, strict: true });
  const release = join(directory, "package");
  await checkRelease(release);
  return release;
}

if (process.argv[1] && resolve(process.argv[1]) === import.meta.filename)
  await checkNpmArchive(process.argv[2] ? resolve(process.argv[2]) : archive);
