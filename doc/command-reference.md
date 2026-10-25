# CanvaToPSD CLI command reference

Generated from Rust CLI 3.0.6 with `canvatopsd discover`. Run `npm run doc:commands` to update this file. Do not edit command tables by hand.

Use this page to inspect every command without installing or executing the CLI. For capabilities, installation availability and pricing boundaries, read [product facts](product-facts.md). For task recovery and credential handling, read [Agent guidance](agent-discovery.md).

Business commands return JSON on stdout; help and version are text. Diagnostics use stderr. Success exits 0; failure exits 1. No `--json` option is needed. Use `--help` on a command for terminal help; generated help aliases are omitted below.

## Global options

| Option                  | Description                                      | Default |
| ----------------------- | ------------------------------------------------ | ------- |
| `--base-url <BASE_URL>` | Web endpoint ending /api, or direct API origin   | —       |
| `--origin <ORIGIN>`     | Origin header matching the configured Web origin | —       |
| `--timeout <TIMEOUT>`   | Per-request timeout in seconds                   | 30      |
| `--help`                | Print help                                       | —       |
| `--version`             | Print version                                    | —       |

## canvatopsd convert

Convert a Canva link and wait for previews; retain one task ID for subsequent commands

```text
canvatopsd convert <url> [OPTIONS]
```

| Argument | Required | Description                                                    |
| -------- | -------- | -------------------------------------------------------------- |
| `url`    | yes      | Canva Share → Public view link, or copied canva.link share URL |

| Option                | Required | Default | Description / allowed values                                      |
| --------------------- | -------- | ------- | ----------------------------------------------------------------- |
| `--email <EMAIL>`     | no       | —       | Optional notification email                                       |
| `--key <KEY>`         | no       | —       | Reuse this UUID after an uncertain response                       |
| `--no-layer-naming`   | no       | false   | Keep original layer names                                         |
| `--dry-run`           | no       | false   | Validate locally without submitting or saving credentials         |
| `--pages <numbers>`   | no       | —       | Design page numbers, e.g. 1,3; required for multi-page designs    |
| `--seconds <SECONDS>` | no       | 300     | Maximum wait after submission; timeout does not cancel conversion |

## canvatopsd create

Submit a design and save task credentials; does not wait

```text
canvatopsd create <url> [OPTIONS]
```

| Argument | Required | Description                                                    |
| -------- | -------- | -------------------------------------------------------------- |
| `url`    | yes      | Canva Share → Public view link, or copied canva.link share URL |

| Option              | Required | Default | Description / allowed values                              |
| ------------------- | -------- | ------- | --------------------------------------------------------- |
| `--email <EMAIL>`   | no       | —       | Optional notification email                               |
| `--key <KEY>`       | no       | —       | Reuse this UUID after an uncertain response               |
| `--no-layer-naming` | no       | false   | Keep original layer names                                 |
| `--dry-run`         | no       | false   | Validate locally without submitting or saving credentials |

## canvatopsd status

Read conversion progress and selected page numbers

```text
canvatopsd status <task> [OPTIONS]
```

| Argument | Required | Description                                      |
| -------- | -------- | ------------------------------------------------ |
| `task`   | yes      | Existing task UUID returned by convert or create |

## canvatopsd wait

Wait for all selected pages; stop at required input or deadline

```text
canvatopsd wait <task> [OPTIONS]
```

| Argument | Required | Description                                      |
| -------- | -------- | ------------------------------------------------ |
| `task`   | yes      | Existing task UUID returned by convert or create |

| Option                | Required | Default | Description / allowed values                                           |
| --------------------- | -------- | ------- | ---------------------------------------------------------------------- |
| `--seconds <SECONDS>` | no       | 300     | Maximum wait in seconds; polls every 5 seconds without cancelling work |

## canvatopsd pages

Discover source pages; a single-page design may start automatically

```text
canvatopsd pages <task> [OPTIONS]
```

| Argument | Required | Description                                      |
| -------- | -------- | ------------------------------------------------ |
| `task`   | yes      | Existing task UUID returned by convert or create |

| Option      | Required | Default | Description / allowed values        |
| ----------- | -------- | ------- | ----------------------------------- |
| `--refresh` | no       | false   | Request fresh source-page discovery |

## canvatopsd select

Select design pages to convert; keep the same task ID

```text
canvatopsd select <task> <pages> [OPTIONS]
```

| Argument | Required | Description                                            |
| -------- | -------- | ------------------------------------------------------ |
| `task`   | yes      | Existing task UUID returned by convert or create       |
| `pages`  | yes      | Comma-separated positive design page numbers, e.g. 1,3 |

| Option        | Required | Default | Description / allowed values                             |
| ------------- | -------- | ------- | -------------------------------------------------------- |
| `--key <KEY>` | no       | —       | Reuse the submission UUID after an uncertain response    |
| `--dry-run`   | no       | false   | Inspect the command input without submitting this action |

## canvatopsd preview

List previews, or save one with --out

```text
canvatopsd preview <task> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option                            | Required | Default      | Description / allowed values                                                                     |
| --------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------ |
| `--page <PAGE>`                   | no       | —            | Design page number; required when multiple pages were converted                                  |
| `--out <OUT>`                     | no       | —            | Write to a new output file; existing files are never overwritten                                 |
| `--artifact-page <ARTIFACT_PAGE>` | no       | —            | Explicit internal artifact page ID; normally use --page instead                                  |
| `--file <FILE>`                   | no       | preview.webp | Preview artifact to list or save Allowed: preview.webp, preview-layers.webp, preview-layers.json |

## canvatopsd download

Synchronize payment and download a PSD or fonts; verify size and SHA-256

```text
canvatopsd download <task> --out <OUT> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option                            | Required | Default       | Description / allowed values                                                       |
| --------------------------------- | -------- | ------------- | ---------------------------------------------------------------------------------- |
| `--page <PAGE>`                   | no       | —             | Design page number; required when multiple pages were converted                    |
| `--out <OUT>`                     | yes      | —             | Write to a new output file; existing files are never overwritten                   |
| `--artifact-page <ARTIFACT_PAGE>` | no       | —             | Explicit internal artifact page ID; normally use --page instead                    |
| `--file <FILE>`                   | no       | page-0001.psd | Paid PSD or font artifact to download Allowed: page-0001.psd, fonts.zip, fonts.txt |

## canvatopsd files

List paid PSD and font files

```text
canvatopsd files <task> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option          | Required | Default | Description / allowed values                                    |
| --------------- | -------- | ------- | --------------------------------------------------------------- |
| `--page <PAGE>` | no       | —       | Design page number; required when multiple pages were converted |

## canvatopsd checkout

Create a hosted payment link; share checkoutUrl unchanged with the user

```text
canvatopsd checkout <task> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option          | Required | Default | Description / allowed values                                    |
| --------------- | -------- | ------- | --------------------------------------------------------------- |
| `--page <PAGE>` | no       | —       | Design page number; required when multiple pages were converted |
| `--dry-run`     | no       | false   | Inspect the command input without submitting this action        |

## canvatopsd payment

Read and synchronize payment state

```text
canvatopsd payment <task> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option          | Required | Default | Description / allowed values                                    |
| --------------- | -------- | ------- | --------------------------------------------------------------- |
| `--page <PAGE>` | no       | —       | Design page number; required when multiple pages were converted |

## canvatopsd confirm

Confirm authorized payment; may capture approved funds

```text
canvatopsd confirm <task> [OPTIONS]
```

| Argument | Required | Description                                                              |
| -------- | -------- | ------------------------------------------------------------------------ |
| `task`   | yes      | Original task UUID returned by convert; reuse it for every selected page |

| Option          | Required | Default | Description / allowed values                                    |
| --------------- | -------- | ------- | --------------------------------------------------------------- |
| `--page <PAGE>` | no       | —       | Design page number; required when multiple pages were converted |

## canvatopsd doctor

Check API protocol and configuration; submits no conversion jobs

```text
canvatopsd doctor [OPTIONS]
```

## canvatopsd resolve

Parse a public Canva link or private task link locally; no network

```text
canvatopsd resolve <url> [OPTIONS]
```

| Argument | Required | Description                                                      |
| -------- | -------- | ---------------------------------------------------------------- |
| `url`    | yes      | Public Canva URL or private task URL to parse without networking |

## canvatopsd init

Save endpoint settings; defaults to prod

```text
canvatopsd init [url] [OPTIONS]
```

| Argument | Required | Description                                                |
| -------- | -------- | ---------------------------------------------------------- |
| `url`    | no       | HTTPS service endpoint, or loopback HTTP; defaults to prod |

## canvatopsd auth

Import, export or inspect local task credentials

```text
canvatopsd auth <COMMAND> [OPTIONS]
```

## canvatopsd auth import

Import a credential JSON file or private task link file for this endpoint

```text
canvatopsd auth import <file> [OPTIONS]
```

| Argument | Required | Description                                        |
| -------- | -------- | -------------------------------------------------- |
| `file`   | yes      | Credential JSON or complete private task link file |

## canvatopsd auth export

Write one secret task credential backup; does not overwrite files

```text
canvatopsd auth export <task> <file> [OPTIONS]
```

| Argument | Required | Description                                                             |
| -------- | -------- | ----------------------------------------------------------------------- |
| `task`   | yes      | Task UUID with a direct local credential; export its parent when needed |
| `file`   | yes      | New secret credential backup file                                       |

## canvatopsd auth status

Show credential availability without printing secrets

```text
canvatopsd auth status [OPTIONS]
```

## canvatopsd auth clear

Remove endpoint credentials and retry tokens; does not revoke server grants

```text
canvatopsd auth clear [OPTIONS]
```

## canvatopsd skill

Read, inspect or install bundled Agent instructions; offline

```text
canvatopsd skill <COMMAND> [OPTIONS]
```

## canvatopsd skill show

Read bundled SKILL.md as JSON without installation

```text
canvatopsd skill show [OPTIONS]
```

## canvatopsd skill status

Compare installed Skill files with this binary's bundled instructions

```text
canvatopsd skill status [agent] [OPTIONS]
```

| Argument | Required | Description                                                                |
| -------- | -------- | -------------------------------------------------------------------------- |
| `agent`  | no       | codex, claude-code, cursor, copilot or agents; use --dir for other targets |

| Option        | Required | Default | Description / allowed values                                 |
| ------------- | -------- | ------- | ------------------------------------------------------------ |
| `--dir <DIR>` | no       | —       | Custom skills parent directory, e.g. /project/.agents/skills |

## canvatopsd skill install

Install bundled Skill; --replace replaces the entire existing directory

```text
canvatopsd skill install [agent] [OPTIONS]
```

| Argument | Required | Description                                                                |
| -------- | -------- | -------------------------------------------------------------------------- |
| `agent`  | no       | codex, claude-code, cursor, copilot or agents; use --dir for other targets |

| Option        | Required | Default | Description / allowed values                                        |
| ------------- | -------- | ------- | ------------------------------------------------------------------- |
| `--dir <DIR>` | no       | —       | Custom skills parent directory, e.g. /project/.agents/skills        |
| `--replace`   | no       | false   | Replace the entire installed Skill directory, including local edits |

## canvatopsd operator

Direct API operator reads; requires CANVATOPSD_OPERATOR_TOKEN

```text
canvatopsd operator <COMMAND> [OPTIONS]
```

## canvatopsd operator task-list

List tasks; server pagination is 100 tasks per page

```text
canvatopsd operator task-list [OPTIONS]
```

| Option              | Required | Default | Description / allowed values                                  |
| ------------------- | -------- | ------- | ------------------------------------------------------------- |
| `--offset <OFFSET>` | no       | 0       | Server task-list offset; each page contains at most 100 tasks |

## canvatopsd operator notification-status

Read task notification records

```text
canvatopsd operator notification-status <task> [OPTIONS]
```

| Argument | Required | Description                                               |
| -------- | -------- | --------------------------------------------------------- |
| `task`   | yes      | Existing task UUID to inspect with operator authorization |

## canvatopsd request

Send an explicit POST command; side effects depend on the API action

```text
canvatopsd request <path> --body <BODY> [OPTIONS]
```

| Argument | Required | Description                            |
| -------- | -------- | -------------------------------------- |
| `path`   | yes      | POST command path such as /task/status |

| Option          | Required | Default | Description / allowed values                                     |
| --------------- | -------- | ------- | ---------------------------------------------------------------- |
| `--body <BODY>` | yes      | —       | File containing the JSON request body                            |
| `--operator`    | no       | false   | Use CANVATOPSD_OPERATOR_TOKEN authorization                      |
| `--out <OUT>`   | no       | —       | Write to a new output file; existing files are never overwritten |

## canvatopsd discover

Offline JSON command index generated from actual command definitions

```text
canvatopsd discover [command] [OPTIONS]
```

| Argument  | Required | Description                                                          |
| --------- | -------- | -------------------------------------------------------------------- |
| `command` | no       | Quoted command path such as "skill install"; omit for the whole tree |
