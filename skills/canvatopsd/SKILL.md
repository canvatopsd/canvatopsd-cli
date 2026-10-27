---
name: canvatopsd
description: Convert public Canva links to editable layered Photoshop PSDs, or retrieve previews and paid files for an existing CanvaToPSD task. Use for Canva to PSD, Canva to Photoshop and layered PSD export. Requires shell access; does not edit existing PSDs or convert arbitrary images/PDFs.
metadata:
  author: CanvaToPSD
  version: "3.2.1"
  homepage: https://canvatopsd.com/ai-agent
---

# CanvaToPSD

Use `canvatopsd` with Node.js 22.16+. No account, API key or email is required. Business commands return JSON: success is `{"ok":true,"data":...}`; failure is `{"ok":false,"error":{"code":"...","message":"..."}}` with exit code 1. Help/version are text. Do not add `--json`.

At the start of an Agent task, run `canvatopsd update check` once before hosted commands. Report whether a newer stable CLI is available. The check returns current/latest versions and an installation command when needed; it does not install anything. A local version ahead of npm is not a downgrade request. If the check fails, report that uncertainty separately; do not claim the CLI is current.

Run `canvatopsd --help` or `canvatopsd discover COMMAND` for exact arguments. Discovery works offline. Run `doctor` to check the selected service before submitting work; report failures without creating a probe conversion. If the CLI is missing, read [references/operations.md](references/operations.md).

## Convert, preview and deliver

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --agent AGENT_NAME
canvatopsd preview TASK
canvatopsd checkout TASK
# After the user pays:
canvatopsd download TASK --out ./design.psd
```

The bundled instructions describe Rust CLI 3.2.1; use its returned `taskId` as the same TASK throughout. Existing tasks start with `status TASK` or `wait TASK`, without creating another conversion.

`preview TASK` returns `previewUrl`. Present that link unchanged so the user can view the design in the Web interface. Web handles interactive layer previews. To save a whole-page image, use `preview TASK --out ./preview.webp`. Preview links last seven days; run preview again for a fresh link to the same task. These private links are intended for delivery to the requesting user; do not publish them or extract their access information.

For multiple pages, use `convert URL --pages 1,3`. If the user has not specified pages, read `error.availablePages`, ask which pages to convert, run `select TASK PAGE_NUMBERS`, then `wait TASK`. Page numbers start at 1. `preview TASK` opens the task overview; add `--page 3` for a selected page. Checkout, download and image saving require `--page N` when several pages were converted.

`checkout` returns a hosted `checkoutUrl`. Present it unchanged. The user chooses and completes payment on the Web page. `download` checks payment before saving the PSD. `payment status TASK` checks payment separately; `payment confirm` may capture funds and requires payment confirmation authorization.

A timeout preserves the task and returns `error.taskId`; resume with `wait TASK --seconds 300`. If a task requires input, inspect `status TASK` and report the relevant page and message. Do not repeatedly submit it.

Submission commands print a submission key before sending. After an uncertain response, reuse `--key UUID` with the identical URL, pages, email, locale and layer-naming settings. Keep the same task and key when recovering; neither is a replacement for saved task access.

When submitting from an AI Agent, pass `--agent NAME` using the actual Agent name (for example `codex`, `claude-code`, `cursor` or `copilot`). Add `--agent-version VERSION` only when the actual application version is known; do not substitute a model name or guess a version. Plain terminal use omits these options. The CLI reports its own version automatically. This attribution is recorded on first submission and inherited by selected pages.

Optional `--email EMAIL` enables notifications. `--locale CODE` sets task-page and notification language; omission uses English. It does not change PSD content or layer naming. `--no-layer-naming` skips model naming while retaining readable local names. Inspect command discovery for allowed values; report unsupported options without silently changing the request.

Deliver the saved PSD path and preview link when available. Report dimensions, layers, fonts and validation only from returned data or actual inspection. Preview readiness, payment, verified download and Photoshop reopening are separate milestones. File writes refuse overwrites. Repository-generated output belongs under `.tmp/artifacts/`.

## Installation and recovery

Read [references/operations.md](references/operations.md) for installation, font downloads, task-access backup/import or Skill updates. Task access is managed by the CLI; do not print local credentials or attach access backups to chat. Use supported commands rather than constructing Web URLs, calling internal APIs or handling preview resources.
