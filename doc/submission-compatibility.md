# Submission compatibility

Since CLI 3.1.5, creating a task omits `locale` unless explicitly requested. Sending a fixed `locale: "en"` previously failed against deployed strict schemas with `command_invalid`.

CLI 3.1.8 adds optional `--locale CODE` for task pages and notifications. Omission still sends no field, preserving compatibility with older APIs. Explicit locale requires the API language contract; `doctor` verifies the base task-token protocol, not this optional capability. Rejected requests fail explicitly without dropping the option or retrying.

The provider owns `api/contracts/ui-locale.schema.json`. The independent CLI keeps an identical snapshot at `src/ui-locale.schema.json` for Clap validation, help and offline discovery. Compare it with the provider when updating language support; standalone builds need no API source checkout or TypeScript dependency. Additional-page requests inherit the parent language.

An uncertain submission must reuse its original key, saved credential and identical URL, email, locale, layer-naming settings and page selection. Do not change locale while retrying an existing submission.

Native platform acceptance covers offline command discovery, submission dry-run and credential privacy. Hosted conversion and payment acceptance are separate checks; do not create another conversion merely to validate installation.
