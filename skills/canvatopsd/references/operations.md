# Detailed operations

## Installation and release identity

Install the published Rust CLI with `npm install -g @canvatopsd/cli`, then run `canvatopsd --version`, `canvatopsd skill install codex` and `canvatopsd doctor`. Node.js 22.16+ is needed for the npm launcher; end users do not need Rust. Supported platforms are macOS ARM64, Linux ARM64/x64 and Windows ARM64/x64.

Website downloads follow an independent release. Check the website's [release metadata](https://canvatopsd.com/agent/release.json) before treating that download as the installed Rust package.

Use `claude-code`, `cursor`, `copilot`, `agents` or `--dir PATH` for another Agent. The execution environment's PATH, OS/CPU and Node runtime may differ from the user's terminal. The installed package's `doc/index.md` links product facts and the generated command reference; `canvatopsd discover` returns their package-relative paths.

## Preview and font files

- `preview TASK` lists previews; add `--out FILE` to save one. Use `--file preview-layers.json` or `preview-layers.webp` for layer previews. `files TASK` lists paid files; `download` defaults to PSD, or use `--file fonts.zip` / `fonts.txt`. Downloads verify size and SHA-256 and refuse overwrites.

## Credentials and recovery

No account, API key, or email is required. Creation generates and saves a task-specific random token **before** sending the request. The server stores only its hash; the CLI saves its task Cookie. Retries with `--key` reuse the saved token, so a lost create response does not lose access. `auth status` lists saved submission keys without exposing tokens.

Email is optional via `create URL --email EMAIL` and only enables notifications. Optional `--locale CODE` on convert/create sets task page and notification language; omission leaves the API default. Additional pages inherit the parent language. Inspect `discover convert` for supported codes. Keep locale unchanged when reusing a submission key. Explicit locale requires a language-aware API; doctor only checks the base task protocol. Without email, use `status` / `wait` for progress and requested input. Ready/payment does not revoke the original credential. Parent credentials authorize direct children. Only the target task or its parent credential is sent. `select`, `pages`, and parent `status` record authorization relations; for children created before 2.2, run `status PARENT_TASK` once to populate them.

Back up one task with `canvatopsd auth export TASK /absolute/task-access.json`; restore on a new client with `canvatopsd auth import /absolute/task-access.json`. Export the parent when a child uses parent authorization. Exports are secret 0600 files; do not print them. Imports require the same endpoint. Existing private email-link files, including `zh-tw` and `pt-br` URL prefixes, can also be imported. `auth clear` removes cookies, saved retry tokens, and parent-child authorization relations for the endpoint. With no email, losing local credentials and all backups prevents self-service recovery.

The API must support optional email and `taskToken`; an old API rejects the new request. Do not work around this with placeholder email addresses or account-level keys.

Configuration is `~/.config/canvatopsd/config.json`; use `CANVATOPSD_CONFIG` to isolate agents when desired. Configuration updates use a file lock and merge into freshly read data. `ELOCKED` means another command is updating it: wait for that command, then retry; retain `--key` when a submission outcome is uncertain. Only endpoint settings and credentials (including retry tokens) are persisted. `CANVATOPSD_BASE_URL` overrides the saved endpoint. Operator commands require a direct API origin and `CANVATOPSD_OPERATOR_TOKEN`; direct API customer commands also require `CANVATOPSD_PROXY_TOKEN`. Web calls need neither secret. Operator auth does not replace task credentials.

Stdout success is `{ "ok": true, "data": ... }`; failure is `{ "ok": false, "error": { "code": "...", "message": "..." } }` with exit 1. Help/version are text. Report explicit errors rather than retrying mutations. `wait TASK --seconds 300` bounds the entire wait including HTTP requests and stops when input/selection is required.

For uncovered actions, first inspect `canvatopsd request --help` and the public usage guide at https://canvatopsd.com/agent/doc/usage.md. Use `canvatopsd request /command/path --body /absolute/request.json` (`--operator` only for operator commands). Raw calls are POST and may write. Do not auto-pay, resend mail, retry failed execution, or restart services without the user's request.

Install this Skill with `canvatopsd skill install codex`, `claude-code`, `cursor`, `copilot`, or `agents`; custom parent directory: `skill install --dir PATH`. Keep generated output in a user-selected directory.
