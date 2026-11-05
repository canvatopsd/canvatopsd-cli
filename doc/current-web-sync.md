# CLI 3.4.10 and current Web

This release follows the R2-backed Web product after retirement of the independent Native API and Ops services. It does not modify or deploy Web, Payment, Runner or Email.

## Behavior changes

- New submissions require a delivery email and retain the initial selected pages; omission selects page 1.
- Private task links accept the entry, payment and result routes, including locale prefixes and selected-page queries.
- Payment reads and confirmation use task-level prepayment commands. Checkout preserves the server-issued link and expiry; Web chooses the payment or result view. No per-page order fallback remains. A requested page is validated before payment confirmation.
- PSD download and file listing use the current `page-1.psd` delivery name. Transfer size and SHA-256 checks still apply.
- Status exposes Runner progress and keeps parent cancellation or attention states ahead of child progress.
- Source-page discovery only reads recorded information. Retired operator commands and the unsupported refresh flag are removed; offline discovery documents the current task-ID format and command effects.
- Standalone source builds carry exact Web-owned protocol snapshots. Source checks verify snapshot equality when Web source is present.
- Credential acceptance uses an isolated home directory and the actual native runtime state path.

## Local validation

Source formatting, Cargo check and Clippy, final release inspection, native macOS ARM64 acceptance and local HTTP contract acceptance are required. The contract acceptance covers link import, payment scope and confirmation boundaries, status precedence, current delivery filenames, transfer integrity and no-overwrite behavior. It uses synthetic responses matching the Web handlers and does not perform a real conversion or payment.

Local packaging and installation follow build, final-directory inspection, candidate archive inspection and installation from the checked immutable archive. Other operating systems, hosted conversion, live payment and public distribution require their separate acceptance and release steps. This change introduces no custom traits, generic abstractions, custom macros, shared locks or background tasks.
