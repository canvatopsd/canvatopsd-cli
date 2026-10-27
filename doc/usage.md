# CanvaToPSD CLI

Use stable task commands for conversion, previews, payment and downloads. The CLI uses the hosted service by default. It requires Node.js 22.16+ and shell access; no account or API key is needed. Business results are JSON; do not add `--json`.

## Check for updates

Run `canvatopsd update check` before starting a new Agent task. It reports currentVersion, latestVersion, updateAvailable and a version-specific installation command. This read-only npm check uses no task credentials or local business state. A local release ahead of npm reports `ahead`; it is not downgraded. Registry failures are explicit and do not prove that the installed CLI is current. Checking does not install an update.

## Normal workflow

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
canvatopsd preview TASK
canvatopsd checkout TASK
# After the user pays:
canvatopsd download TASK --out ./design.psd
```

Keep the returned `taskId` as the same TASK throughout. `preview` returns a private `previewUrl` that opens the Web task page. Share it unchanged with the requesting user. Web handles task access, layer rendering and preview interaction. The link lasts seven days; repeat preview to refresh it without creating another task or payment.

To save a whole-page image, use `preview TASK --out ./preview.webp`. The CLI does not expose layer metadata, atlas files or internal file-page identifiers.

## Multiple pages

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --pages 1,3
canvatopsd preview TASK --page 3
canvatopsd checkout TASK --page 3
canvatopsd download TASK --page 3 --out ./page3.psd
```

Page numbers start at 1. `preview TASK` opens the overview even when several pages were selected. Image saving, checkout and download require a specific page when several were converted. Missing selections return `availablePages` and `nextAction`.

For a design with several pages and no initial choice, use `pages TASK`, ask the user which pages to convert, then run `select TASK 1,3` and `wait TASK`. Do not create another task.

## Status and recovery

`status TASK` reports selected pages, progress, payment state and required input. `wait TASK --seconds 300` waits within a deadline; timeout preserves the task. A multi-page conversion is ready when all selected pages are ready. When a page needs input, report its page number and message.

Submission commands print a submission key before sending. After an uncertain response, retry with the same key and identical URL, pages, email, locale and layer-naming settings. To continue waiting alone, use wait. `--dry-run` validates submission options without submitting or saving credentials. `checkout --dry-run` checks the live selected page without creating a payment.

## Payment and files

`checkout TASK` returns a hosted `checkoutUrl`. Give it to the user unchanged. Opening it establishes task access and shows previews, prices and payment controls. The user chooses the payment method there. The entry lasts seven days; repeat checkout for a fresh entry to the same order.

`download` checks payment before saving. `payment status TASK` checks it separately; `payment confirm TASK` may capture approved funds and requires payment authorization.

Downloads default to PSD. Use `--format fonts` for a ZIP archive or `--format font-list` for a text list. Add `--page N` for a selected design page. `files TASK --page N` lists available download formats and sizes. Transfers verify size and SHA-256, preserve existing output files and leave no final file on integrity failure. A successful transfer does not establish Photoshop reopening or visual fidelity.

## Optional settings and task access

When submitting from an AI Agent, pass `--agent NAME` using the actual Agent name (for example `codex`, `claude-code`, `cursor` or `copilot`). Add `--agent-version VERSION` only when the actual application version is known; do not substitute a model name or guess a version. Plain terminal use omits these options. The CLI reports its own version automatically. This attribution is recorded on first submission and inherited by selected pages.

`--email EMAIL` enables notifications. `--locale CODE` sets task-page and notification language, independently of PSD content and layer naming; omission uses English. `--no-layer-naming` skips model naming while keeping readable local names. Additional pages inherit parent settings. Inspect `discover convert` for supported values. Older services may reject explicit locale; report the error without changing the input or retrying a mutation.

The CLI saves task access automatically. Do not print its local configuration or credentials. Back up with `auth export TASK FILE`; restore with `auth import FILE`. Export the parent task when it authorizes selected pages. A complete private task-link file can also be imported. Imports require the same service. See [task recovery](../skills/canvatopsd/references/operations.md).

For HTTP 401/403, restore matching task access. For `ELOCKED`, wait for the other settings writer to finish. A task ID or submission key alone cannot restore access. `auth clear` removes local access for the selected service and should only be used intentionally.

## Discovery and Skill installation

```sh
canvatopsd --help
canvatopsd discover
canvatopsd discover preview
canvatopsd skill show
canvatopsd skill status codex
canvatopsd skill install codex
```

Default discovery lists product commands and their actual arguments, defaults, allowed values and effects. It works offline without credentials. Help and version are text; business commands return `{"ok":true,"data":...}` or `{"ok":false,"error":...}` with exit code 1. Diagnostics use stderr.

Skill installation supports codex, claude-code, cursor, copilot, agents or a custom `--dir`. Review local edits before `--replace`; it replaces the whole installed directory. Start a fresh Agent session after an update.

## Maintainer access and release compatibility

`discover --advanced` includes maintenance commands and connection settings. Explicit discovery of a maintenance command is also supported. Those commands are outside the ordinary conversion workflow; raw requests retain API response structures. See the [complete command reference](command-reference.md) and maintainer help when an explicitly requested operation requires them.

CLI 3.2 replaces preview resource selection with a Web entry and whole-page image saving. Download selection uses `--format psd|fonts|font-list`; internal `--artifact-page` and artifact `--file` options have been removed. Existing task IDs, credentials and submission keys remain usable. The preview-link API and Web entry handler must be deployed before this CLI is publicly distributed. An older service fails explicitly; the CLI does not substitute checkout or build a URL locally.

`doctor` checks the base service protocol. It does not verify every optional capability, task authorization, Runner readiness, payment or Photoshop compatibility. Local, npm and website releases can differ; identify each channel separately before quoting availability.
