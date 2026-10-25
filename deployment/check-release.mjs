import { createHash } from "node:crypto";
import { checkPrivacy } from "./check-privacy.mjs";
import { readdir, readFile, open, mkdir, mkdtemp } from "node:fs/promises";
import { join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { build, unit, project, revision } from "./paths.mjs";
import { targets, hostTarget } from "./targets.mjs";
import { commandReference } from "./command-reference.mjs";

function checkNativeHeader(bytes, target) {
  const arm = target.endsWith("arm64");
  if (target.startsWith("darwin-"))
    return (
      bytes.readUInt32LE(0) === 0xfeedfacf &&
      bytes.readUInt32LE(4) === (arm ? 0x100000c : 0x1000007)
    );
  if (target.startsWith("linux-"))
    return (
      bytes.subarray(0, 4).equals(Buffer.from([0x7f, 0x45, 0x4c, 0x46])) &&
      bytes[4] === 2 &&
      bytes[5] === 1 &&
      bytes.readUInt16LE(18) === (arm ? 183 : 62)
    );
  if (bytes.toString("ascii", 0, 2) !== "MZ") return false;
  const offset = bytes.readUInt32LE(0x3c);
  return (
    offset + 6 <= bytes.length &&
    bytes.toString("ascii", offset, offset + 4) === "PE\0\0" &&
    bytes.readUInt16LE(offset + 4) === (arm ? 0xaa64 : 0x8664)
  );
}
export async function checkRelease(
  directory = build,
  { publishing = false } = {},
) {
  directory = resolve(directory);
  let files = 0;
  async function inspect(folder) {
    for (const item of await readdir(folder, { withFileTypes: true })) {
      const path = join(folder, item.name);
      if (item.isDirectory()) await inspect(path);
      else if (item.isFile()) {
        const file = await open(path, "r");
        const bytes = Buffer.alloc(128);
        try {
          await file.read(bytes, 0, bytes.length, 0);
        } finally {
          await file.close();
        }
        if (
          bytes
            .toString()
            .startsWith("version https://git-lfs.github.com/spec/v1")
        )
          throw new Error(`Unrestored LFS pointer: ${path}`);
        files++;
      } else throw new Error(`Unexpected release entry: ${path}`);
    }
  }
  await inspect(directory);
  await checkPrivacy(directory);
  for (const file of [
    "README.md",
    "LICENSE.md",
    "bin/canvatopsd.mjs",
    "doc/usage.md",
    "doc/agent-discovery.md",
    "doc/index.md",
    "doc/product-facts.md",
    "doc/command-reference.md",
    "skills/canvatopsd/SKILL.md",
    "skills/canvatopsd/agents/openai.yaml",
    "skills/canvatopsd/references/operations.md",
    "package.json",
    "release.json",
  ])
    await readFile(join(directory, file));
  const source = JSON.parse(await readFile(join(unit, "package.json"), "utf8"));
  const manifest = JSON.parse(
    await readFile(join(directory, "package.json"), "utf8"),
  );
  const release = JSON.parse(
    await readFile(join(directory, "release.json"), "utf8"),
  );
  if (
    manifest.name !== source.name ||
    manifest.version !== source.version ||
    release.version !== source.version ||
    release.runtime !== "rust" ||
    manifest.private ||
    manifest.dependencies ||
    manifest.bin?.canvatopsd !== "bin/canvatopsd.mjs" ||
    manifest.publishConfig?.access !== "public" ||
    manifest.publishConfig?.registry !== "https://registry.npmjs.org/"
  )
    throw new Error("Invalid Rust npm release manifest");
  if (
    !Array.isArray(release.targets) ||
    !release.targets.length ||
    new Set(release.targets).size !== release.targets.length
  )
    throw new Error("Missing or duplicate native targets");
  const runtimeEntries = await readdir(join(directory, "runtime"));
  if (
    JSON.stringify([...runtimeEntries].sort()) !==
    JSON.stringify([...release.targets].sort())
  )
    throw new Error("Runtime contents differ from declared release targets");
  for (const target of release.targets) {
    if (!targets[target]) throw new Error(`Unknown native target: ${target}`);
    const path = join(directory, "runtime", target, targets[target].executable);
    const handle = await open(path, "r");
    const bytes = Buffer.alloc(65536);
    try {
      await handle.read(bytes, 0, bytes.length, 0);
    } finally {
      await handle.close();
    }
    if (!checkNativeHeader(bytes, target))
      throw new Error(`Binary format/architecture does not match ${target}`);
    if (publishing) {
      const receipt = JSON.parse(
        await readFile(
          join(directory, "runtime", target, "acceptance.json"),
          "utf8",
        ),
      );
      const digest = createHash("sha256")
        .update(await readFile(path))
        .digest("hex");
      if (
        receipt.target !== target ||
        receipt.version !== source.version ||
        receipt.revision !== release.revision ||
        receipt.sha256 !== digest ||
        receipt.acceptance !== "native-offline"
      )
        throw new Error(`Platform acceptance does not match ${target}`);
    }
  }
  if (publishing && release.revision !== revision())
    throw new Error("Release revision differs from source checkout");
  if (publishing)
    for (const target of source.canvatopsd.publishTargets)
      if (!release.targets.includes(target))
        throw new Error(
          `Public npm release missing ${target}; collect platform builds and prepare the complete release first`,
        );
  if (!release.targets.includes(hostTarget))
    throw new Error(
      `Cannot execute final artifact on this host: ${hostTarget} is missing`,
    );
  const entry = join(directory, "bin/canvatopsd.mjs");
  const run = (args) =>
    execFileSync(process.execPath, [entry, ...args], {
      cwd: "/",
      encoding: "utf8",
    }).trim();
  if (run(["--version"]) !== source.version)
    throw new Error("Native version differs from npm version");
  run(["--help"]);
  const discovery = JSON.parse(run(["discover", "skill", "install"]));
  if (
    JSON.stringify(discovery) !==
    JSON.stringify(JSON.parse(run(["discover", "skill install"])))
  )
    throw new Error("Quoted and natural discovery paths differ");
  const payment = JSON.parse(run(["discover", "payment", "confirm"]));
  if (payment.data.command.effects.mayCapturePayment !== true)
    throw new Error("Payment confirmation effects are missing");
  const previewHelp = run(["preview", "--help"]);
  if (previewHelp.includes("--artifact-page") || previewHelp.includes("--file"))
    throw new Error("Default preview help exposes internal artifact options");
  if (
    !discovery.ok ||
    discovery.data.version !== source.version ||
    !discovery.data.command.options.some((o) => o.flags === "--replace")
  )
    throw new Error("Native discovery failed");
  const conversion = JSON.parse(run(["discover", "convert"]));
  if (
    !conversion.data.command.options.some(
      (o) => o.flags === "--pages <numbers>",
    )
  )
    throw new Error("Conversion workflow missing");
  const rootDiscovery = JSON.parse(run(["discover"]));
  const expectedReference = await commandReference(rootDiscovery.data);
  if (
    expectedReference !==
    (await readFile(join(directory, "doc/command-reference.md"), "utf8"))
  )
    throw new Error(
      "Command reference differs from the native CLI; run npm run doc:commands before packaging",
    );
  if (
    rootDiscovery.data.product?.package !== manifest.name ||
    rootDiscovery.data.product?.homepage !== manifest.homepage
  )
    throw new Error("Agent product discovery differs from npm metadata");
  const skill = JSON.parse(run(["skill", "show"]));
  if (
    skill.data.content !==
    (await readFile(join(directory, "skills/canvatopsd/SKILL.md"), "utf8"))
  )
    throw new Error(
      "Embedded Skill differs from packaged instructions; rebuild native binaries",
    );
  const skillChecks = join(project, ".tmp/artifacts/cli-skill-check");
  await mkdir(skillChecks, { recursive: true });
  const skillParent = await mkdtemp(join(skillChecks, `${source.version}-`));
  run(["skill", "install", "--dir", skillParent]);
  for (const file of [
    "SKILL.md",
    "agents/openai.yaml",
    "references/operations.md",
  ]) {
    const embedded = await readFile(join(skillParent, "canvatopsd", file));
    const packaged = await readFile(join(directory, "skills/canvatopsd", file));
    if (!embedded.equals(packaged))
      throw new Error(`Embedded Skill file differs from package: ${file}`);
  }
  console.log(
    JSON.stringify({
      checked: directory,
      version: source.version,
      runtime: "rust",
      targets: release.targets,
      files,
      lfsPointers: 0,
    }),
  );
}
if (process.argv[1] && resolve(process.argv[1]) === import.meta.filename)
  await checkRelease();
