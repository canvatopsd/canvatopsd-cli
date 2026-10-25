# Build and release the Rust CLI

Release toolchain: Rust 1.97.1 with rustfmt and Clippy, pinned in `rust-toolchain.toml` and CI. Formatting invokes `cargo-fmt` directly so an older rustup cargo subcommand cannot silently override the selected formatter. Other requirements: Git, Node.js 22.16+, npm and the target platform's compiler/linker. Linux GNU builds require OpenSSL development libraries and pkg-config, and matching runtime libraries.

Run commands from the CLI source checkout:

```sh
npm ci --cache .tmp/npm-cache
npm run format
npm run doc:commands
npm run check
npm pack
npm run deploy:local
```

When this checkout is nested inside another repository, place npm's cache in that repository's root `.tmp/`. The build scripts choose the owning repository's `.tmp/` for Cargo output, candidates and verification records. This maintainer document is not included in the npm package.

## Public metadata and privacy

`build.rs` selects only the product identity, capabilities, input/output descriptions, documentation paths and service requirements needed by `discover`. The full source manifest is not embedded in the executable. Do not add unpublished repository addresses, personal email addresses or credentials to public metadata.

`deployment/cargo-environment.mjs` applies path-prefix remapping to both application and dependency compilation. Release validation inspects every final file, including native binaries, for developer home paths, internal repository/service identifiers and complete private-key blocks. It also rejects unresolved Git LFS pointers, mismatched architectures, stale command documentation and differing embedded Skill files. A failed check blocks packaging and installation. A detector result does not replace a review of newly added content.

Keep Git authorship suitable for public distribution. This repository uses the public CanvaToPSD support identity. Do not commit local backups, credentials, customer artifacts or internal acceptance records.

## Platform artifacts

The default build targets the host. Local development acceptance runs on macOS ARM64; each CI release independently verifies its declared native targets. `deployment/targets.mjs` defines the five intended targets: macOS ARM64, Linux ARM64/x64 and Windows ARM64/x64; public npm publication requires all of them. Each platform must be built and exercised with its matching toolchain and runtime.

Set `CANVATOPSD_BUILD_TARGETS` to a comma-separated list of target names. For externally built, accepted binaries, set `CANVATOPSD_NATIVE_INPUT` to a directory containing `<target>/canvatopsd` or `<target>/canvatopsd.exe`. The final privacy and format checks also apply to externally supplied binaries.

## Versions and publication

`npm version patch` synchronizes Cargo.toml and Cargo.lock; regenerate command documentation with `npm run doc:commands`. Published or archived versions are immutable. Use a new version for any changed executable, documentation or Skill.

`npm pack` enforces build → final-directory checks → candidate archive extraction and checks → immutable archive integrity verification. `deploy:local` installs that checked archive. `npm publish --dry-run` checks completeness without publishing; actual publication requires scope access and an explicit release action. Existing checked archives are reused without rebuilding under the same version.

Public release metadata contains the version, public source revision, runtime, targets and build time. Working-tree file lists and developer-machine details are not published. Old private development archives belong outside this public checkout. If packaging is interrupted, run `npm run postpack` to clean generated files.

## Automated public release

See [public release checklist](releasing.md). The workflow builds and executes
all five platforms before aggregation. `npm run accept:platform` records native
offline acceptance; `npm run release:npm -- --dry-run` checks the complete archive,
and `npm run release:npm` publishes that exact archive. Public publication requires
native acceptance receipts for every target and the same source revision.
