import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { unit } from "./paths.mjs";
const metadata = JSON.parse(await readFile(join(unit, "package.json"), "utf8"));
execFileSync(process.execPath, [join(unit, "deployment/build.mjs")], {
  env: {
    ...process.env,
    CANVATOPSD_BUILD_TARGETS: metadata.canvatopsd.publishTargets.join(","),
  },
  stdio: "inherit",
});
