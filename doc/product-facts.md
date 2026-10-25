# CanvaToPSD CLI: product facts

CanvaToPSD is a hosted Canva-to-PSD conversion service with a local Rust command-line client. It converts public Canva design links to layered Adobe Photoshop PSD files. Supported text stays editable; images and vector artwork become separate raster image layers. It serves designers moving Canva work into Photoshop and AI agents automating that workflow.

## Identity and access

| Field                          | Fact                                                                            |
| ------------------------------ | ------------------------------------------------------------------------------- |
| Product                        | CanvaToPSD                                                                      |
| CLI command                    | `canvatopsd`                                                                    |
| npm package name               | `@canvatopsd/cli`                                                               |
| Agent Skill name               | `canvatopsd`                                                                    |
| Product and Agent setup        | [canvatopsd.com/ai-agent](https://canvatopsd.com/ai-agent)                      |
| Hosted API endpoint            | `https://canvatopsd.com/api`                                                    |
| Support                        | [support@canvatopsd.com](mailto:support@canvatopsd.com)                         |
| Ordinary hosted authentication | Per-task credentials created and stored locally; no account or API key required |
| Email                          | Optional; enables notifications                                                 |
| Conversion and preview         | Free                                                                            |
| PSD download                   | Paid; consult [current pricing](https://canvatopsd.com/pricing)                 |

CanvaToPSD is an independent service. The CLI's use of Canva and Photoshop formats does not imply an official Canva or Adobe CLI.

## Input and output

Accepted source URLs are Canva **Share → Public view link → Copy link** URLs and copied `https://canva.link/...` share links. Existing conversions are addressed by their task UUID. Private editor links, arbitrary image files, PDFs and existing PSDs are not conversion inputs to this CLI.

The CLI can retrieve a layered PSD, WebP preview, JSON/WebP layer preview, font ZIP and font-list TXT when available for the task. Static single-page and selected multi-page designs are supported. Design page numbers start at 1. Multiple selected pages produce separate page results; preserve the original task ID and add `--page N` when retrieving a selected page.

Vector paths remain rasterized. Font availability and effects may differ from Canva. Video, animation, interactive exports and offline Canva rendering are not supported. A verified download confirms size and SHA-256 transfer integrity, not pixel-identical rendering or Photoshop reopening.

## Availability: source release and hosted distribution

Source release and CI acceptance are version-specific. Website distribution was last checked on 2026-10-24:

| Channel                  | State                                                                                                                        |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| This source release      | Rust 3.1.5; accepted local target is macOS Apple Silicon (`darwin-arm64`)                                                    |
| Local install archive    | `releases/canvatopsd-cli-3.1.5.tgz` after the checked release build                                                          |
| Website CLI distribution | The public [release metadata](https://canvatopsd.com/agent/release.json) reported 2.3.3; it is an earlier JavaScript release |
| Public npm registry      | The package metadata request returned HTTP 404; this Rust release has not been published                                     |
| Other Rust platforms     | Build definitions exist; native execution acceptance is pending                                                              |

Installing from the website does not currently install this Rust release. Registry commands such as `npx --yes @canvatopsd/cli` are intended for use after public npm publication. A directory cloned onto a local machine and an installed Skill make the tool discoverable there; they do not make it searchable on the public web.

For Rust npm archives, Node.js 22.16+ is required only for the native launcher; Rust is not required by end users. Direct native binaries do not require Node. The hosted 2.3.3 JavaScript CLI requires Node.js 24.5+. Hosted conversions require an internet connection regardless of client runtime.

## Agent interface and payment

Business commands emit `{"ok":true,"data":...}` on stdout, or `{"ok":false,"error":{"code":"...","message":"..."}}` with exit code 1. Help and version are text; progress diagnostics use stderr. No `--json` flag is needed.

`canvatopsd discover` returns the actual command tree plus product capabilities, supported input/output descriptions, documentation paths and service requirements. `canvatopsd skill show` returns the bundled instructions. Both work offline without reading task credentials.

The shortest workflow is `convert URL` → `preview TASK --out FILE` → `checkout TASK` → user payment → `download TASK --out FILE`. Share the hosted `checkoutUrl` unchanged. `payment confirm` can capture approved funds. A request to convert a design does not itself imply permission to pay.

## References for quotation and implementation

Use this page for product identity and boundaries, [the command reference](command-reference.md) for exact syntax, [Agent guidance](agent-discovery.md) for task routing, and [current pricing](https://canvatopsd.com/pricing) for payment amounts. Use returned data or actual artifact inspection for design-specific dimensions, layer counts, fonts and delivery status.
