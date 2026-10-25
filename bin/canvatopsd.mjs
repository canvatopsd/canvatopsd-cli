#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

const target = `${process.platform}-${process.arch}`;
const executable =
  process.platform === "win32" ? "canvatopsd.exe" : "canvatopsd";
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
