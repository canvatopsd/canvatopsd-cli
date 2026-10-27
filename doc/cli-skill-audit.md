# CLI and Skill synchronization audit

CLI 3.1.8 was checked against the API command schemas and implementation on 2026-10-27.

## Changes

- Add optional `--locale CODE` to convert/create, using the provider-owned language schema snapshot. Omission still sends no locale field, preserving older strict API compatibility. Explicit locale requires a language-aware API; the base protocol doctor does not negotiate this capability.
- Recognize child-task `inputRequired` values of 0/1 from the API database projection, as well as boolean values. Status now exposes required input and wait stops for attention.
- Correct the Skill checkout flow: hosted links establish task access and show the task page with PayPal/Creem controls. Opening the link does not automatically redirect to a payment provider.
- Remove the obsolete claim that the Rust CLI depends on the TypeScript contracts package. Clarify discovery output and identical-input requirements for uncertain submission recovery.
- Synchronize Skill and product metadata versions during npm version changes. Final artifact validation rejects version differences in these documents.

## Validation

Source checks include rustfmt, Clippy with warnings denied, JavaScript syntax checks and Markdown formatting. A temporary local API simulation exercised 35 language values and 39 requests across task creation, page discovery/selection, status, attention, parent authorization, previews, checkout, payment reads, file listing and delivery. Download size/hash checks and refusal to overwrite existing files were exercised; a failed integrity check left no final file. Offline submission validation created no credential file.

The existing production read-only protocol endpoint passed doctor. No real conversion jobs, payment creation, fund capture or email sending were used for this audit. A protocol check does not establish production locale support or Runner readiness. Local simulation is not acceptance of the full hosted conversion workflow or Photoshop editability.

The local release follows build, final artifact/archive checks, then npm installation. This audit targets macOS ARM64; other native platforms require their own acceptance receipts before public npm publication. Website distribution and API releases remain separate deployment units.
