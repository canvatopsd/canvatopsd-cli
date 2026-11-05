#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { nativeExecutable } from "./native-executable.mjs";
import metadata from "../package.json" with { type: "json" };

const target = `${process.platform}-${process.arch}`;
if (!metadata.canvatopsd.publishTargets.includes(target)) {
  console.log(
    JSON.stringify({
      ok: false,
      error: {
        code: "unsupported_platform",
        message: `Unsupported platform: ${target}. Supported platforms: ${metadata.canvatopsd.publishTargets.join(", ")}.`,
      },
    }),
  );
  process.exit(1);
}
const executable = nativeExecutable(
  metadata.version,
  metadata.canvatopsd.environment,
  target,
);
const path = fileURLToPath(
  new URL(`../runtime/${target}/${executable}`, import.meta.url),
);
if (!existsSync(path)) {
  console.log(
    JSON.stringify({
      ok: false,
      error: {
        code: "native_binary_missing",
        message: `This package does not contain a Rust binary for ${target}; install a release that includes your platform.`,
      },
    }),
  );
  process.exit(1);
}
const result = spawnSync(path, process.argv.slice(2), {
  stdio: "inherit",
  windowsHide: true,
});
if (result.error) {
  console.log(
    JSON.stringify({
      ok: false,
      error: { code: "native_launch_failed", message: result.error.message },
    }),
  );
  process.exit(1);
}
if (result.signal) process.kill(process.pid, result.signal);
else process.exit(result.status ?? 1);
