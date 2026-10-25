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

## Availability and platform requirements

| Channel             | State                                                                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Public npm registry | [@canvatopsd/cli](https://www.npmjs.com/package/@canvatopsd/cli) is published; install with `npm install -g @canvatopsd/cli`                              |
| Supported platforms | macOS Apple Silicon (ARM64), Linux ARM64/x64, Windows ARM64/x64                                                                                           |
| Native acceptance   | Each release requires matching native receipts for all five platforms in [CI](https://github.com/canvatopsd/canvatopsd-cli/actions/workflows/release.yml) |
| Website download    | Follows the independent Web release; inspect its [release metadata](https://canvatopsd.com/agent/release.json) separately                                 |

The npm launcher requires Node.js 22.16+. End users do not need Rust; direct native binaries do not need Node. Linux requires glibc and OpenSSL 3. macOS Intel and Alpine/musl are unsupported. Hosted conversion requires an internet connection regardless of client runtime.

Use `canvatopsd --version` and `canvatopsd discover` to identify the installed package. Website documentation can describe a different release.

## Agent interface and payment

Business commands emit `{"ok":true,"data":...}` on stdout, or `{"ok":false,"error":{"code":"...","message":"..."}}` with exit code 1. Help and version are text; progress diagnostics use stderr. No `--json` flag is needed.

`canvatopsd discover` returns the actual command tree plus product capabilities, supported input/output descriptions, documentation paths and service requirements. `canvatopsd skill show` returns the bundled instructions. Both work offline without reading task credentials.

The shortest workflow is `convert URL` → `preview TASK --out FILE` → `checkout TASK` → user payment → `download TASK --out FILE`. Share the hosted `checkoutUrl` unchanged. `payment confirm` can capture approved funds. A request to convert a design does not itself imply permission to pay.

## References for quotation and implementation

Use this page for product identity and boundaries, [the command reference](command-reference.md) for exact syntax, [Agent guidance](agent-discovery.md) for task routing, and [current pricing](https://canvatopsd.com/pricing) for payment amounts. Use returned data or actual artifact inspection for design-specific dimensions, layer counts, fonts and delivery status.
