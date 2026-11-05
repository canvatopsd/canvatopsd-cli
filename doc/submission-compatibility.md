# Submission compatibility

## Current Web product

CLI 3.4.10 uses Web-owned `/api/task/*` commands and the independent Payment service through Web. The Native API, legacy API Worker and Ops services are retired; old operator commands are removed. Current task IDs are case-sensitive, 22-character base64url UUIDv7 values, not hyphenated UUID strings. Historical submissions remain retained; this release does not migrate them to the current product.

`create` and `convert` require `--email EMAIL`. `convert --pages 1,3` selects and prices those pages in the initial submission; omission selects page 1. Payment covers the entire selected page set. There is no later page selection or per-page purchase API. `payment status` reads the saved payment state; `payment confirm` can synchronize an authorized payment and capture approved funds. Downloads check current delivery eligibility without synchronizing payment.

PSD delivery uses `page-1.psd` within each selected page's result. The CLI translates this to the public `psd` format; users continue to select design pages with `--page N`.

Web-issued private task links can use `/task/TASK`, `/task/TASK/payment` or `/task/TASK/result`, including supported locale prefixes and page queries. Link import exchanges the original fragment through Web. Checkout reuses the server-issued private link unchanged; Web routes unpaid users to payment and paid users to results.

`pages` reads discovery already recorded on the task. It does not start capture, refresh discovery or change the selected pages. `status` includes the task's progress and preserves cancellation and attention states.

## Submission identity and recovery

The CLI reports its own channel and version. Agent name and application version are explicit optional inputs. An uncertain submission must reuse its original task ID, saved credential and identical URL, email, locale, layer-naming settings and page selection. See [R2 reception and recovery](submission-inbox.md). Rejected requests fail explicitly; do not drop fields or retry with different inputs.

`--locale CODE` sets task-page and notification language. Omission leaves the API default. The provider owns `web/contracts/ui-locale.schema.json`; the CLI snapshot at `src/ui-locale.schema.json` must match it. The CLI also keeps exact snapshots of the provider-owned Rust task-ID and native installation contracts in `src/contracts/` and `deployment/native-build.rs`. Source checks compare these snapshots with Web when built inside the product checkout. Standalone public builds use the bundled snapshots without requiring Web source.

## Validation boundaries

`doctor` checks the base task-token protocol. It does not verify task access, Runner readiness, payment capture or delivery. Native acceptance checks offline discovery, dry-run submissions and credential privacy in an isolated home directory. Local contract acceptance uses synthetic HTTP responses matching the current Web handlers; it does not establish live payment or hosted conversion acceptance.
