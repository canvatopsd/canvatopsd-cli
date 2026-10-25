# Public release checklist

## Source repository

Publish only this independent CLI repository and its sanitized Git history.
The owning product repository and local `.tmp/` backups are not public artifacts.
The public repository is https://github.com/canvatopsd/canvatopsd-cli and its
default branch is `main`. Package metadata points to this repository and its
Issues page. Push this independent checkout with `git push origin main` after
reviewing the final source and history. Keep the public description and topics
consistent with the supported capabilities.

The source is publicly readable under the existing proprietary policy in
[LICENSE.md](../LICENSE.md). It is not advertised as open source.

## Platform builds

The workflow at `.github/workflows/release.yml` is relative to this standalone
repository. It runs six native runners, checks source and final artifacts,
executes offline discovery, Skill installation and submission dry-run, and
records the binary SHA-256, source revision and version in `acceptance.json`.
The aggregation job checks every receipt before its npm publication dry-run.
These receipts establish native offline acceptance, not hosted conversion or
payment acceptance. Linux x64 builds use Ubuntu 22.04 (glibc 2.35/OpenSSL 3);
Linux ARM64 uses Ubuntu 24.04 (glibc 2.39/OpenSSL 3). Alpine/musl is unsupported.
The workflow has to run successfully before cross-platform support is claimed.

GitHub artifact upload does not preserve Unix execute bits. Aggregation restores
them before checking and packing. External binaries must include matching
`<target>/acceptance.json` files generated on their actual platform by
`npm run build` followed by `npm run accept:platform`.

## npm setup

The package is `@canvatopsd/cli`, public access. Confirm ownership of the
`@canvatopsd` organization and the package name using the intended npm account.
Configure npm Trusted Publishing for the actual GitHub owner, repository,
`release.yml` workflow and `npm` environment. The first publication may require
an authenticated maintainer bootstrap before that package can be configured.
Use a local npm login for that bootstrap; never commit an npm token.

Protect the `npm` GitHub environment and the release branch as appropriate for
the maintainers. The manual publish job runs only from `main`; update this
literal if the actual release branch is changed. Push and pull request builds
never publish. No public remote or account permissions are assumed by source.

## Prepare and publish

1. Complete all source, metadata, README and Skill edits before versioning.
2. Use a fresh immutable version (`npm version patch --no-git-tag-version`),
   regenerate `npm run doc:commands`, format, check, then commit the source.
3. Run the workflow. All six platforms must pass and the aggregate archive must
   pass privacy, architecture, version, documentation and acceptance checks.
4. Download the checked archive and exercise the installed CLI against prod:
   `doctor`, one authorized design submission, preview, status/resume, and a
   download after an explicitly arranged checkout. Check PSD opening/editability
   when making those claims. Record acceptance privately under `.tmp/artifacts/`.
5. Publish from that same source revision using `npm run release:npm` or rerun
   the manual workflow with `publish` enabled. The workflow rebuilds and checks
   its own final archive before publishing it. For the exact previously accepted
   archive, use the local command. It publishes the inspected `.tgz` directly.
6. Verify registry version and integrity, then install from the public registry
   into a clean npm prefix and run version, discovery and dry-run commands.
7. Update availability statements and website distribution through the Web
   unit's separate build, final-artifact check and explicit deployment.

`npm run release:npm -- --dry-run` performs the same local release checks without
publishing. Missing native receipts or mismatched revisions fail explicitly.
Adding platform binaries or changing any packaged content requires a new version;
existing single-platform 3.0.2 remains immutable.
