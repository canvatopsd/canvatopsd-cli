---
name: canvatopsd
description: Convert public Canva links to editable layered Photoshop PSDs, or retrieve previews and paid files for an existing CanvaToPSD task. Use for Canva to PSD, Canva to Photoshop and layered PSD export. Requires shell access; does not edit existing PSDs or convert arbitrary images/PDFs.
metadata:
  author: CanvaToPSD
  version: "3.1.6"
  homepage: https://canvatopsd.com/ai-agent
---

# CanvaToPSD

Use `canvatopsd` with Node.js 22.16+ (npm launcher; conversion commands run in Rust). Results are already JSON; do not add `--json`. Help/version are text. Success is `{ "ok": true, "data": ... }` (exit 0); failure is `{ "ok": false, "error": { "code": "...", "message": "..." } }` (exit 1). Diagnostics use stderr.

## Start

- Run `canvatopsd --help` for the overview, `canvatopsd COMMAND --help` for arguments/examples, or `canvatopsd discover skill install` for a machine-readable command definition. `discover` works offline without configuration or credentials.
- Default service is **prod** `https://canvatopsd.com/api`. Use `--base-url http://127.0.0.1:8080/api` explicitly for local. Run `doctor` against the intended endpoint before submitting work: it verifies the read-only `/task/protocol/read` API and required task-token protocol. Report failures without submitting a probe job.
- If the CLI is missing, read the installation section in [references/operations.md](references/operations.md). No account, API key or email is required for ordinary Web calls. `convert --email EMAIL` only enables notifications.

## Convert and deliver

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
canvatopsd preview TASK --out ./preview.webp
canvatopsd checkout TASK
# After the user pays:
canvatopsd download TASK --out ./design.psd
```

The bundled instructions describe Rust CLI 3.1.6; the same short workflow exists in 2.3+. `convert` submits the design, discovers pages and waits up to 300 seconds for previews. Use its returned `taskId` as the same TASK for every later command. Never look up or switch to internal conversion IDs.

For multiple pages, use `convert URL --pages 1,3`. It does not guess which pages the user wants. To preview, check out or download one selected design page, add its number, e.g. `preview TASK --page 3 --out ./page3.webp`. Page numbers are 1-based. If several pages were converted, omitting `--page` reports the available numbers.

A timeout preserves the task and returns `error.taskId`; resume with `wait TASK --seconds 300`. Existing tasks start with `status TASK` or `wait TASK`, without creating another task. If discovery finds several pages without a selection, read `error.availablePages`, ask the user to choose, run `select TASK PAGE_NUMBERS`, then `wait TASK`. File and payment results retain that TASK as `data.taskId` and expose `data.page`. Keep the same TASK throughout. `status` reports selected page numbers and their readiness.

`convert`, `create` and `select` print a submission key before sending. After an uncertain `convert` response, reuse its `submitKey` via `--key UUID` with identical URL and page selection. `convert` uses the task ID as its page-selection submission key, so resuming the same conversion does not create duplicate pages. Both keys remain separate from secret task access credentials.

`checkout` returns a CanvaToPSD-hosted payment link. Present its `checkoutUrl` unchanged: it establishes task access in whichever browser opens it, then continues to PayPal, including for existing orders. Do not substitute an earlier raw PayPal link. The hosted link is valid for 7 days; run `checkout` again for a fresh link to the same order. `download` synchronizes payment before fetching the file; `payment status TASK` provides a separate status check; `payment confirm` may capture funds and requires payment confirmation authorization.

Deliver the saved PSD link and preview when available. Report dimensions, layers, fonts and validation only when supported by actual returned data or inspection. Readiness, preview, payment and verified download are separate milestones; download alone does not prove Photoshop reopening or semantic editability. File writes refuse overwrites. In this repository, generated output belongs under `.tmp/artifacts/`.

## Conditional operations

Read [references/operations.md](references/operations.md) only for credential backup/import, missing task access, direct API/operator calls, configuration details, font/layer preview files, or CLI installation. It includes retry and endpoint constraints. Raw task credentials and exported access files are secret; do not attach them to chat. The opaque hosted `checkoutUrl` is intended for delivery to the paying user.

For Skill discovery use `canvatopsd skill status codex`; install with `skill install codex` or `skill install --dir ./.agents/skills`. Review existing local edits before `--replace`, which replaces the whole Skill directory. Start a new Agent session to refresh an already loaded Skill catalog.
