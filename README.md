# Canva to PSD CLI — CanvaToPSD

Convert public Canva design links into editable, layered Photoshop PSD files from your terminal or AI agent. CanvaToPSD preserves supported editable text and separates images and artwork into layers. Conversion runs on the hosted service, with free previews and paid downloads.

[npm](https://www.npmjs.com/package/@canvatopsd/cli) · [Agent setup](https://canvatopsd.com/ai-agent) · [Real examples](https://canvatopsd.com/showcases) · [Pricing](https://canvatopsd.com/pricing) · [Documentation](doc/index.md)

## Install

Requires Node.js 22.16 or later. No Rust compiler, Photoshop installation, Canva login, CanvaToPSD account or API key is required.

```sh
npm install -g @canvatopsd/cli
canvatopsd --version
canvatopsd --help
```

Run without a global installation:

```sh
npx --yes @canvatopsd/cli --help
```

Supported platforms: macOS Apple Silicon (ARM64), Linux ARM64/x64 and Windows ARM64/x64. macOS Intel and Alpine/musl are unsupported. Linux requires glibc and OpenSSL 3. See [platform requirements](doc/development.md#platform-artifacts) for details.

## Convert a Canva design

In Canva, choose **Share → Public view link → Copy link**. Copied `canva.link` share URLs are also accepted.

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
# Replace TASK with the returned taskId:
canvatopsd preview TASK --out ./preview.webp
canvatopsd checkout TASK
# After the user completes payment through the returned checkoutUrl:
canvatopsd download TASK --out ./design.psd
```

Keep the same task ID throughout. Conversion and previews are free; PSD downloads require payment at the [current price](https://canvatopsd.com/pricing). An email address is optional for notifications.

For multiple pages, choose the design page numbers explicitly:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --pages 1,3
canvatopsd preview TASK --page 3 --out ./page3.webp
canvatopsd checkout TASK --page 3
canvatopsd download TASK --page 3 --out ./page3.psd
```

Page numbers start at 1. If conversion times out, resume with `canvatopsd wait TASK`; do not submit another conversion just to continue waiting. Existing tasks start with `status TASK`. Downloads verify size and SHA-256 and never overwrite existing files.

## Use with an AI agent

The package includes a Skill for Codex, Claude Code, Cursor, GitHub Copilot and other agents with shell access.

```sh
canvatopsd skill install codex
canvatopsd skill status codex
```

Use `claude-code`, `cursor`, `copilot` or `agents` instead of `codex`, or install into a project with `canvatopsd skill install --dir ./.agents/skills`. Start a new agent session after installation.

Agents can inspect the actual command definitions offline:

```sh
canvatopsd discover convert
canvatopsd discover payment confirm
canvatopsd skill show
```

Business commands return JSON without a `--json` flag: success is `{"ok":true,"data":...}`; failure is `{"ok":false,"error":...}` with exit code 1. Help and version are text. Discovery includes parameter types, limits and side effects. Page-selection errors include available page numbers and the next action.

Present `checkoutUrl` unchanged to the user. `payment confirm` may capture approved funds and requires payment authorization. Keep task credentials and credential exports private.

## Output and limits

| Content                   | PSD result                                                       |
| ------------------------- | ---------------------------------------------------------------- |
| Supported text            | Editable text layers                                             |
| Photos and images         | Separate image layers                                            |
| Vector artwork            | Separate raster image layers; vector paths are not editable      |
| Static multi-page designs | A PSD for each selected converted page                           |
| Fonts and effects         | May differ from Canva; inspect the preview and downloaded result |

Video, animation, interactive content, arbitrary image/PDF conversion and editing existing PSDs are outside this CLI's scope. The CLI runs locally; conversion and delivery require an internet connection. Transfer verification does not by itself establish pixel-identical rendering or native Photoshop compatibility.

## Documentation and support

- [Installation and conversion walkthrough](doc/npm-readme.md)
- [Complete command reference](doc/command-reference.md)
- [Agent selection and discovery](doc/agent-discovery.md)
- [Usage, recovery and troubleshooting](doc/usage.md)
- [Product facts and distribution channels](doc/product-facts.md)
- [Build and development](doc/development.md) · [Release checklist](doc/releasing.md)
- [Issues](https://github.com/canvatopsd/canvatopsd-cli/issues) · [Support](mailto:support@canvatopsd.com)

The CLI, bundled Agent Skill and included documentation are licensed under the [MIT License](LICENSE.md). Use of the hosted service is governed by its [service terms](https://canvatopsd.com/terms). Report security issues privately to support@canvatopsd.com; omit task tokens, credential files and private links from public reports.
