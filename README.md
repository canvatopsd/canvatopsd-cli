# Canva to PSD CLI — CanvaToPSD

Convert a public Canva design link into an editable, layered Adobe Photoshop PSD from a terminal or AI agent. CanvaToPSD CLI is written in Rust and distributed through the npm package **`@canvatopsd/cli`**, with the command **`canvatopsd`** and a bundled Agent Skill.

Use it to move a Canva design into Photoshop, automate Canva-to-PSD exports, select pages from a presentation, retrieve conversion previews, and download paid PSD and font files. Supported text remains editable; images and vector artwork become separate image layers.

[CI builds](https://github.com/canvatopsd/canvatopsd-cli/actions/workflows/release.yml) · [Product and Agent setup](https://canvatopsd.com/ai-agent) · [Conversion examples](https://canvatopsd.com/showcases) · [Pricing](https://canvatopsd.com/pricing) · [Command reference](doc/command-reference.md) · [Agent discovery](doc/agent-discovery.md)

## Install

The Rust 3.0 release is prepared locally; it is **not yet published to the public npm registry**. Install a checked archive for your platform:

```sh
npm install -g ./releases/canvatopsd-cli-3.1.2.tgz
canvatopsd --version
canvatopsd --help
```

Once the complete release is published to npm:

```sh
npm install -g @canvatopsd/cli
# Or run without a global install:
npx --yes @canvatopsd/cli --help
```

npm installation requires Node.js 22.16 or later. End users do not need Rust or Cargo. The npm launcher starts a bundled native executable without downloading code during installation. A directly installed native binary runs without Node.js. The conversion service requires an internet connection.

Release targets are macOS on Apple Silicon (ARM64), and Linux/Windows on ARM64 or x64. macOS Intel (x64) is unsupported. Every public npm
release requires native CI acceptance for all five targets; check the linked CI
run for the exact source revision. Local development acceptance runs on macOS
Apple Silicon. Linux uses GNU libc and OpenSSL 3, with Ubuntu 22.04 for x64 and
Ubuntu 24.04 for ARM64 as the CI baseline; Alpine/musl is unsupported.

The website currently distributes an earlier JavaScript CLI, verified as 2.3.3 on 2026-10-24. Its Node requirement is 24.5+. Check [hosted release metadata](https://canvatopsd.com/agent/release.json) and `canvatopsd --version` before treating a website install as this Rust release.

## Convert Canva to Photoshop

No account, API key or email is required for the ordinary hosted workflow. Preview conversion is free; downloadable results require payment at the current [pricing](https://canvatopsd.com/pricing).

```sh
canvatopsd doctor
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
canvatopsd preview TASK --out /absolute/preview.webp
canvatopsd checkout TASK
# After the user completes payment:
canvatopsd download TASK --out /absolute/design.psd
```

Replace `CANVA_PUBLIC_VIEW_URL` with Canva's **Share → Public view link → Copy link** and `TASK` with the returned `taskId`. Copied `https://canva.link/...` share links are also accepted. Keep the same task ID throughout the workflow. Present `checkoutUrl` unchanged to the paying user.

For a multi-page Canva design:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --pages 1,3
canvatopsd preview TASK --page 3 --out /absolute/page3.webp
canvatopsd checkout TASK --page 3
canvatopsd download TASK --page 3 --out /absolute/page3.psd
```

Page numbers start at 1. A timeout preserves the task: resume with `canvatopsd wait TASK`. Downloads refuse to overwrite existing files and verify the size and SHA-256 supplied by the delivery receipt.

## Find the right Canva export tool

Use this CLI for requests such as “export a Canva design to an editable Photoshop PSD”, “convert selected Canva presentation pages to layered PSD files”, or “Canva 转 PSD / Canva 导出 Photoshop 分层文件”. For an existing CanvaToPSD task, start with `status TASK`; a Canva URL is needed only when submitting a new design. Canva-to-PDF, image background removal and editing an existing PSD require other tools.

[Product facts](doc/product-facts.md) gives a compact reference for quoting capabilities, formats and service requirements. [The full command reference](doc/command-reference.md) is generated from the actual Rust command definitions, so an agent or crawler can read every command without executing the CLI.

## AI agents: Codex, Claude Code, Cursor and Copilot

Agents with shell access can use the CLI's JSON command interface and bundled Skill. Install instructions for the agent you use:

```sh
canvatopsd skill show
canvatopsd discover
canvatopsd discover skill install
canvatopsd skill install codex
```

Also supported: `claude-code`, `cursor`, `copilot`, and `agents`. For project instructions use `canvatopsd skill install --dir /absolute/project/.agents/skills`. Start a new Agent session after installation to refresh its catalog.

Business commands return JSON without a `--json` flag:

```json
{ "ok": true, "data": { "taskId": "..." } }
```

Errors return `{"ok":false,"error":{"code":"...","message":"..."}}` and exit 1. Help and version are text; progress diagnostics use stderr. `discover` returns product capabilities, input/output descriptions and documentation paths alongside command names, arguments, options, defaults and allowed values from the Rust command definitions. `discover`, `resolve`, `skill show` and submission `--dry-run` work offline without reading task credentials.

For an Agent evaluating this tool, read [capabilities and selection guidance](doc/agent-discovery.md), then [the bundled Skill](skills/canvatopsd/SKILL.md). MCP is not required.

## What is preserved?

| Content             | PSD result                                                    |
| ------------------- | ------------------------------------------------------------- |
| Supported text      | Editable text layers                                          |
| Raster images       | Separate image layers                                         |
| Vector artwork      | Separate image layers; editable vector paths are not retained |
| Static designs      | Single-page and selected multi-page conversions               |
| Fonts and effects   | May differ from Canva; inspect the preview and returned files |
| Video and animation | Not supported                                                 |

A successful verified download confirms transfer integrity. It does not prove pixel-identical rendering, native Photoshop reopening, or semantic editability of every layer. This CLI submits hosted conversions; it does not edit an existing PSD, provide an offline Canva renderer, or convert arbitrary images/PDFs.

## Frequently asked questions

### Can I export Canva to a layered PSD?

Yes. Submit a public Canva view link with `canvatopsd convert`, inspect the preview, complete checkout, then download the layered Photoshop PSD. See [real examples](https://canvatopsd.com/showcases) for supported output.

### Does CanvaToPSD run locally?

The Rust CLI runs locally. Conversion, preview generation, payment and delivery use the hosted service. The default endpoint is `https://canvatopsd.com/api` (prod).

### Can an AI agent use CanvaToPSD without an account?

Yes. Ordinary hosted commands require no account, API key or email. Task access tokens are saved locally. Payment remains a separate user action, and `payment confirm` may capture approved funds.

### How do I recover a pending conversion?

Use `status TASK` or `wait TASK`. After an uncertain submission response, reuse the reported submission key with `--key UUID` and identical inputs. Back up task access with `auth export`; the exported file contains a secret.

### Is the Rust CLI already available on npm?

This repository prepares version 3.1.2 for npm. Public registry publication is a separate release action. Until publication and platform acceptance are complete, use the checked platform archive or build from source.

## Documentation and support

- [Documentation index](doc/index.md)
- [Product facts, availability and output limits](doc/product-facts.md)
- [Complete command reference generated from the Rust CLI](doc/command-reference.md)
- [CLI usage](doc/usage.md)
- [Agent selection, capabilities and machine discovery](doc/agent-discovery.md)
- [Installation and conversion walkthrough](doc/npm-readme.md)
- [Agent Skill](skills/canvatopsd/SKILL.md)
- [Support](mailto:support@canvatopsd.com)

Rust source lives in `src/`; npm's thin launcher in `bin/`; independent build, artifact checks and local installation in `deployment/`; documentation in `doc/`; Agent instructions in `skills/`; immutable version archives in `releases/`. Temporary output belongs in the owning repository's `.tmp/` directory.

## Source and contributions

The source is publicly readable under the existing proprietary policy in
[LICENSE.md](LICENSE.md). It is not released under an open-source license.
Send bug reports or sensitive security reports to support@canvatopsd.com; never
include task tokens, auth exports, customer files or payment details.

Maintainers: the source checkout includes `doc/development.md` and
`doc/releasing.md` for platform acceptance, npm setup, immutable versioning and
release verification. These maintainer guides are excluded from the npm package.

[中文项目介绍](doc/readme.zh-CN.md) · [Software metadata](codemeta.json)

## Command compatibility and task identity

Default help emphasizes common commands. Advanced commands and artifact options
remain available through `discover` and the complete command reference.
Use `payment status TASK` for a payment check and `payment confirm TASK` only
after authorization to capture approved funds. The older `payment TASK` and
`confirm TASK` forms remain supported.

Page-specific file and payment results return the original `taskId` plus the
human-facing `page` number. Preserve the original task ID and the returned
`checkoutUrl`. Errors requiring a page include `availablePages` and `nextAction`,
so agents can request a choice without parsing the error message.
