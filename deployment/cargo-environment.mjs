import { homedir } from "node:os";
import { resolve } from "node:path";
import { cargoTarget, project } from "./paths.mjs";

export function cargoEnvironment() {
  const flags =
    process.env.CARGO_ENCODED_RUSTFLAGS !== undefined
      ? process.env.CARGO_ENCODED_RUSTFLAGS.split("\x1f").filter(Boolean)
      : (process.env.RUSTFLAGS || "").split(/\s+/).filter(Boolean);
  const mappings = [
    [homedir(), "/build/user"],
    [
      resolve(process.env.CARGO_HOME || resolve(homedir(), ".cargo")),
      "/build/dependencies",
    ],
    [project, "/build/source"],
  ];
  return {
    ...process.env,
    CARGO_TARGET_DIR: cargoTarget,
    CARGO_ENCODED_RUSTFLAGS: [
      ...flags,
      ...mappings.map(
        ([source, target]) => `--remap-path-prefix=${source}=${target}`,
      ),
    ].join("\x1f"),
  };
}
