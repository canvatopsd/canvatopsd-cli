# Installation and task recovery

## Install and identify the CLI

Install with `npm install -g @canvatopsd/cli`, then run `canvatopsd --version`, `canvatopsd skill install codex` and `canvatopsd doctor`. Node.js 22.16+ is required. Supported platforms are macOS ARM64, Linux ARM64/x64 and Windows ARM64/x64.

Website downloads are released separately. Check the website's [release metadata](https://canvatopsd.com/agent/release.json) when identifying that distribution. A locally installed release can be newer than the public registry or website.

Use `claude-code`, `cursor`, `copilot`, `agents` or `--dir PATH` for another Agent. `skill status codex` compares the installed Skill with the CLI bundle. Review existing local edits before `skill install codex --replace`, which replaces the whole directory. Start a new Agent session after updating.

## Preview and downloads

`preview TASK` returns a Web preview link; `preview TASK --page N` opens a selected page. Share the returned `previewUrl` unchanged. It provides private task access for seven days. Generate a fresh link with the same command when needed. The Web interface owns layer rendering and interaction.

`preview TASK --out FILE` saves a whole-page image. `download TASK --out FILE` saves the PSD. For fonts, use `download TASK --format fonts --out FILE`; use `--format font-list` for a text list. Add `--page N` when multiple pages were converted. Paid downloads verify the transfer and refuse to overwrite existing files.

## Restore task access

The CLI saves task access automatically. On a new machine, import a backup or the user's private task-link file:

```sh
canvatopsd auth export TASK /absolute/task-access.json
canvatopsd auth import /absolute/task-access.json
```

Exports are secret files; do not print or attach them. Export the original parent task when it provides access to selected pages. Imports require the same service. `auth status` reports saved access without printing secrets. Without email or a backup, lost local task access cannot be recovered through self-service. A task ID or submission key alone does not restore access.

For HTTP 401/403, restore the matching task access. `ELOCKED` means another command is updating local settings: let it finish before retrying. For an uncertain submission, retain the original submission key and identical settings. `wait_timeout` means continue with `wait TASK`; it does not cancel the conversion.

Normal commands use the hosted service automatically. Do not change service settings, reconstruct URLs, invoke maintenance commands, retry failed execution or restart services merely to recover a conversion. Report explicit errors and use the supported recovery commands.
