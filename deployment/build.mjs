import { cargoEnvironment } from "./cargo-environment.mjs";
import { cp, mkdir, rm, readFile, writeFile, chmod } from "node:fs/promises";
import { join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { unit, build, cargoTarget, revision } from "./paths.mjs";
import { targets, hostTarget } from "./targets.mjs";

const selected = (process.env.CANVATOPSD_BUILD_TARGETS || hostTarget).split(
  ",",
);
if (
  !selected.length ||
  new Set(selected).size !== selected.length ||
  selected.some((t) => !targets[t])
)
  throw new Error(
    "Invalid CANVATOPSD_BUILD_TARGETS; see deployment/targets.mjs",
  );
const metadata = JSON.parse(await readFile(join(unit, "package.json"), "utf8"));
const cargo = await readFile(join(unit, "Cargo.toml"), "utf8");
if (!cargo.includes(`version = "${metadata.version}"`))
  throw new Error("Cargo and npm versions differ");
await rm(build, { recursive: true, force: true });
await mkdir(build, { recursive: true });
for (const target of selected) {
  const { rust, executable } = targets[target];
  const destination = join(build, "runtime", target);
  await mkdir(destination, { recursive: true });
  let binary;
  if (process.env.CANVATOPSD_NATIVE_INPUT) {
    binary = resolve(process.env.CANVATOPSD_NATIVE_INPUT, target, executable);
  } else {
    execFileSync(
      "cargo",
      [
        "build",
        "--locked",
        "--release",
        "--target",
        rust,
        "--manifest-path",
        join(unit, "Cargo.toml"),
      ],
      {
        cwd: unit,
        env: cargoEnvironment(),
        stdio: "inherit",
      },
    );
    binary = join(cargoTarget, rust, "release", executable);
  }
  await cp(binary, join(destination, executable), { errorOnExist: true });
  await chmod(join(destination, executable), 0o755);
  if (process.env.CANVATOPSD_NATIVE_INPUT) {
    const receipt = resolve(
      process.env.CANVATOPSD_NATIVE_INPUT,
      target,
      "acceptance.json",
    );
    await cp(receipt, join(destination, "acceptance.json"));
  }
}
for (const name of ["bin", "doc", "skills"])
  await cp(join(unit, name), join(build, name), { recursive: true });
for (const name of ["README.md", "LICENSE.md", "codemeta.json"])
  await cp(join(unit, name), join(build, name));
delete metadata.dependencies;
delete metadata.devDependencies;
metadata.scripts = {};
await writeFile(
  join(build, "package.json"),
  JSON.stringify(metadata, null, 2) + "\n",
);
await writeFile(
  join(build, "release.json"),
  JSON.stringify(
    {
      version: metadata.version,
      revision: revision(),
      runtime: "rust",
      targets: selected,
      builtAt: new Date().toISOString(),
    },
    null,
    2,
  ) + "\n",
);
console.log(
  JSON.stringify({
    built: build,
    version: metadata.version,
    targets: selected,
  }),
);
