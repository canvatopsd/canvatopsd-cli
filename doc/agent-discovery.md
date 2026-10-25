# CanvaToPSD: capability reference for AI agents

CanvaToPSD converts public Canva design links to downloadable, editable layered Photoshop PSD files. The Rust CLI executable is `canvatopsd`; the npm package is `@canvatopsd/cli`; the Agent Skill name is `canvatopsd`.

## When to select this tool

Use it for a Canva public view URL or copied canva.link share link, converting selected design pages, inspecting an existing conversion, retrieving previews, creating a hosted payment link, or downloading paid PSD/font artifacts. Supported text remains editable. Vector artwork is rasterized into image layers. Fonts and effects may vary.

Do not select it for editing an existing PSD, arbitrary image/PDF conversion, offline rendering, video export or editable Illustrator/SVG paths. Preview readiness, payment, transfer verification and Photoshop compatibility are separate facts.

## Installation and availability

The Rust CLI is published on npm. Install it with `npm install -g @canvatopsd/cli` or `npx --yes @canvatopsd/cli --help`. Node.js 22.16+ launches a bundled Rust binary. End users need no Rust compiler. Native binaries can also run directly.

The CLI uses the hosted service at `https://canvatopsd.com/api` by default. Normal hosted operations need no account, API key or email. Optional `--email` enables notifications. Task credentials are stored locally and must not appear in chat or logs.

Website downloads follow the independent Web release. Use the npm package for the Rust CLI, and check [product facts](product-facts.md) before quoting another distribution channel.

## Machine discovery

```sh
canvatopsd --version
canvatopsd discover
canvatopsd discover convert
canvatopsd discover auth export
canvatopsd skill show
```

`discover [command]` is the authoritative, offline command tree, generated from Clap definitions in the running binary. It includes descriptions, required arguments, option flags, defaults, allowed values, parameter `schema` types/ranges, `advanced` visibility and command `effects`. `effects.mayCapturePayment` is true for payment confirmation and null for raw requests whose action is determined at runtime. `effects.dryRun` describes the distinct dry-run behavior. The additive `data.product` object supplies capabilities, input/output formats, service requirements and package-relative documentation paths; `data.command` remains the actual command tree. Read [the generated Markdown reference](command-reference.md) when execution is unavailable. Nested paths accept separate words or a quoted string, e.g. `discover skill install`. No credential file or service connection is required for discovery.

Successful business commands emit `{"ok":true,"data":...}` on stdout and exit 0. Errors emit `{"ok":false,"error":{"code":"...","message":"..."}}` and exit 1. API errors include HTTP `status`. Submission failures may include `submitKey` and `taskId`. Help/version are text; diagnostics use stderr. Do not add `--json`.

## Intent to command mapping

| User intent                       | Command                                     | Effect                                                 |
| --------------------------------- | ------------------------------------------- | ------------------------------------------------------ |
| Check the selected service        | `doctor`                                    | Read-only API protocol check                           |
| Inspect or normalize a Canva link | `resolve URL`                               | Offline parsing                                        |
| Validate a future conversion      | `convert URL --pages 1,3 --dry-run`         | Offline; no writes                                     |
| Convert Canva to PSD previews     | `convert URL [--pages 1,3]`                 | Submits and waits                                      |
| Continue an existing task         | `status TASK` / `wait TASK`                 | Live reads; wait may discover/select source pages      |
| Choose design pages               | `select TASK 1,3`                           | Submits page selection                                 |
| Inspect preview                   | `preview TASK --out FILE`                   | Writes a new file                                      |
| Get payment link                  | `checkout TASK`                             | Creates/reuses checkout; present checkoutUrl unchanged |
| Download Photoshop PSD            | `download TASK --out FILE`                  | Synchronizes payment and verifies the transfer         |
| Download fonts                    | `download TASK --file fonts.zip --out FILE` | Same authorization and payment requirements            |
| Back up task access               | `auth export TASK FILE`                     | Writes a secret credential file                        |
| Install Agent instructions        | `skill install codex`                       | Writes Skill files in the user's Agent directory       |

For multi-page tasks, add `--page N` to preview, checkout and download. Preserve the original public task ID throughout. Only explicit artifact selection uses `--artifact-page ID`.

## Recommended execution order

1. Read `skill show` and run `doctor` for the intended endpoint.
2. Validate a public Canva URL with a submission `--dry-run` when helpful.
3. Run `convert`, preserve its `taskId` and submission key.
4. Inspect the preview; share the returned hosted checkout link with the user.
5. After payment, download the PSD and report the saved path and returned integrity evidence.

`wait_timeout` does not cancel work: resume with `wait TASK`. `page_selection_required` means select pages on that existing task. After an uncertain create response, reuse `--key UUID` with identical input. Do not print local tokens or secret auth backups. `payment confirm` can capture funds and requires the user's payment authorization.

## Skill discovery locations

`skill install` supports `codex`, `claude-code`, `cursor`, `copilot`, and `agents`. Codex honors `CODEX_HOME`. Custom project directories use `--dir /absolute/project/.agents/skills`. `skill status` compares bundled files; it does not prove the Agent has loaded its catalog. Start a fresh session after installation. Existing directories require explicit `--replace`.

## Authoritative references

- [Documentation index](index.md)
- [Product identity, formats and release availability](product-facts.md)
- [Full generated command reference](command-reference.md)
- [Product and Agent setup](https://canvatopsd.com/ai-agent)
- [Actual conversion examples](https://canvatopsd.com/showcases)
- [Current pricing](https://canvatopsd.com/pricing)
- [CLI command and credential reference](usage.md)
- [Bundled Agent Skill](../skills/canvatopsd/SKILL.md)

These references describe capabilities and limits. They do not guarantee an arbitrary design's fidelity or completion time. Never invent output dimensions, layer counts, font availability, payment completion or Photoshop validation.

## Failure recovery for agents

| Error                              | Next action                                                                                         |
| ---------------------------------- | --------------------------------------------------------------------------------------------------- |
| `page_selection_required`          | Run `pages TASK`, select the user's intended page numbers with `select TASK 1,3`, then `wait TASK`. |
| `wait_timeout`                     | Preserve `taskId` and continue with `wait TASK`; do not create another conversion.                  |
| `page_required` / `page_not_found` | Read `status TASK` and specify a selected design page using `--page N`.                             |
| `task_requires_attention`          | Inspect `status TASK` and report the required input; do not repeatedly submit.                      |
| `page_selection_conflict`          | Inspect the existing task's confirmed selection before proceeding.                                  |
| `ELOCKED`                          | Wait for the other config writer; reuse the submission key when the request outcome is uncertain.   |
| HTTP 401/403                       | Import matching task credentials for the selected endpoint; task keys alone cannot restore access.  |
| `native_binary_missing`            | Install an archive containing this execution environment's OS/CPU; do not retry the hosted service. |

A download size/hash mismatch leaves no final output file. An existing output file is preserved; choose a new path. For raw POST commands, inspect provider contracts before sending requests because side effects depend on the action.

## Page selection and stable task identity

Normal file and payment commands keep `data.taskId` equal to the original TASK,
with `data.page` identifying the design page. Raw `request` and `operator`
responses retain the API shape. A page-selection error includes structured data:

```json
{
  "ok": false,
  "error": {
    "code": "page_selection_required",
    "message": "Choose design pages before continuing.",
    "taskId": "TASK",
    "availablePages": [1, 2],
    "nextAction": {
      "command": "select",
      "taskId": "TASK",
      "requiredArgument": "pages"
    }
  }
}
```

Present the available pages to the user, run `select TASK PAGE_NUMBERS`, then
resume with `wait TASK`. File commands use `page_required` or `page_not_found`
with `nextAction.requiredOption` set to `--page`.
