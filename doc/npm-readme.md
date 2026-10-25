# Canva to PSD CLI — CanvaToPSD

Convert a **Canva public view link into a layered Photoshop PSD** from your terminal or AI agent. Preview the conversion for free, then pay for the pages you want to download.

CanvaToPSD preserves supported editable text and separates images and artwork into layers. It supports static single-page and multi-page designs.

[Website](https://canvatopsd.com/) · [Agent setup](https://canvatopsd.com/ai-agent) · [Real conversion examples](https://canvatopsd.com/showcases) · [Pricing](https://canvatopsd.com/pricing) · [中文使用文档](https://canvatopsd.com/agent/doc/usage.md)

## Install

Requires **Node.js 22.16 or later**. Supported platforms: macOS Apple Silicon (ARM64), Linux ARM64/x64 and Windows ARM64/x64. macOS Intel is unsupported. No Photoshop installation, Canva account login, API key or CanvaToPSD account is required for the ordinary conversion workflow. The CLI uses the hosted CanvaToPSD service and needs an internet connection for conversion and download.

The Rust 3.0 release is prepared locally and has not been published to npm. Install the checked macOS ARM64 archive from this repository:

```sh
npm install -g ./releases/canvatopsd-cli-3.1.5.tgz
canvatopsd --version
canvatopsd doctor
```

The npm package name is `@canvatopsd/cli`; the executable is `canvatopsd`. Once the package is publicly available on the npm registry, you can also install or run it with:

```sh
npm install -g @canvatopsd/cli
npx --yes @canvatopsd/cli --help
```

## Convert a Canva design

Create a public view link for your design, then run:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
```

Use the returned `taskId` as `TASK` in all following commands:

```sh
canvatopsd preview TASK --out ./preview.webp
canvatopsd checkout TASK
# Open the returned checkoutUrl and complete payment.
canvatopsd download TASK --out ./design.psd
```

For a multi-page design, choose its page numbers explicitly:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --pages 1,3
canvatopsd preview TASK --page 3 --out ./page3.webp
canvatopsd checkout TASK --page 3
canvatopsd download TASK --page 3 --out ./page3.psd
```

If conversion times out, keep the returned task ID and resume with `canvatopsd wait TASK`. Do not submit a second conversion just to continue waiting. Output files are never overwritten.

See [platform availability](product-facts.md) before distributing this version. The hosted download URL may still serve an earlier JavaScript release.

## Use with an AI agent

The package includes a Skill for agents with terminal access:

```sh
canvatopsd skill install codex
canvatopsd skill status codex
```

Use `claude-code`, `cursor`, `copilot` or `agents` instead of `codex`, or specify a custom directory with `--dir PATH`. Start a new agent session after installation.

Example request:

> Use the canvatopsd Skill to convert this Canva public view link. Show me the preview and checkout link, then download the PSD after I pay.

Agents can inspect commands without accessing the network or credentials:

```sh
canvatopsd discover
canvatopsd discover convert
canvatopsd discover skill install
canvatopsd skill show
```

Business commands return JSON: success is `{ "ok": true, "data": ... }`; failure is `{ "ok": false, "error": ... }` with exit code 1. Help and version output are text. Payment requires the user's action; agents should present the returned checkout link rather than automatically paying.

## What is preserved?

| Content               | PSD result                                                         |
| --------------------- | ------------------------------------------------------------------ |
| Supported text        | Editable text layers                                               |
| Photos and images     | Separate image layers                                              |
| Vector artwork        | Separate raster image layers; vector paths are not editable        |
| Multiple static pages | Select pages explicitly and download a PSD for each converted page |
| Fonts and effects     | May differ from Canva; inspect the preview and downloaded result   |

Video, animation and interactive content are not supported. A successful download does not by itself verify native Photoshop reopening or every layer's editability. CanvaToPSD is an independent service.

See actual results: [Fashion studio social post](https://canvatopsd.com/showcase/fashion-social) and [Autumn mood photo post](https://canvatopsd.com/showcase/autumn-photo). Each case provides source and output previews with dimensions and layer information.

## Pricing and task access

Conversion and preview are free; PSD downloads are paid. See [current pricing](https://canvatopsd.com/pricing). No subscription is required. An email address is optional for notifications.

The CLI stores task access credentials locally in `~/.config/canvatopsd/config.json`. Keep this file and credential exports private. Use `canvatopsd auth export --help` to back up access before moving to another machine.

## Documentation and support

- [Documentation index](index.md)
- [Product facts and channel availability](product-facts.md)
- [Complete command reference](command-reference.md)
- [Full usage guide / 中文使用文档](usage.md)
- [Bundled Agent Skill](../skills/canvatopsd/SKILL.md)
- [Bundled detailed operations](../skills/canvatopsd/references/operations.md)
- [Create a Canva public view link](https://canvatopsd.com/canva-sharing)
- [Service privacy](https://canvatopsd.com/privacy)
- [Support](mailto:support@canvatopsd.com)

The installed package also includes `doc/usage.md`, `skills/canvatopsd/SKILL.md` and its detailed reference. Use `canvatopsd COMMAND --help` for the exact command options. When reporting an issue, include the CLI version, Node.js version, operating system and error code; omit task tokens, credential files and private links.
