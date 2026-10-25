import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { build } from "./paths.mjs";
import { targets, hostTarget } from "./targets.mjs";
import { checkRelease } from "./check-release.mjs";

await checkRelease(build);
const release = JSON.parse(await readFile(join(build, "release.json"), "utf8"));
const binary = join(
  build,
  "runtime",
  hostTarget,
  targets[hostTarget].executable,
);
const result = JSON.parse(
  execFileSync(
    binary,
    [
      "convert",
      "https://www.canva.com/design/EXAMPLE/EXAMPLE/view",
      "--dry-run",
    ],
    { encoding: "utf8" },
  ),
);
if (!result.ok) throw new Error("Native submission dry-run failed");
const receipt = {
  target: hostTarget,
  version: release.version,
  revision: release.revision,
  sha256: createHash("sha256")
    .update(await readFile(binary))
    .digest("hex"),
  acceptance: "native-offline",
};
await writeFile(
  join(build, "runtime", hostTarget, "acceptance.json"),
  JSON.stringify(receipt, null, 2) + "\n",
);
console.log(JSON.stringify(receipt));
