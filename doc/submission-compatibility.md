# Submission compatibility

Since CLI 3.1.5, creating a task omits `locale` unless explicitly requested. Sending a fixed `locale: "en"` previously failed against deployed strict schemas with `command_invalid`.

CLI 3.1.8 adds optional `--locale CODE` for task pages and notifications. Omission still sends no field, preserving compatibility with older APIs. Explicit locale requires the API language contract; `doctor` verifies the base task-token protocol, not this optional capability. Rejected requests fail explicitly without dropping the option or retrying.

The provider owns `api/contracts/ui-locale.schema.json`. The independent CLI keeps an identical snapshot at `src/ui-locale.schema.json` for Clap validation, help and offline discovery. Compare it with the provider when updating language support; standalone builds need no API source checkout or TypeScript dependency. Additional-page requests inherit the parent language.

An uncertain submission must reuse its original key, saved credential and identical URL, email, locale, layer-naming settings and page selection. Do not change locale while retrying an existing submission.

Native platform acceptance covers offline command discovery, submission dry-run and credential privacy. Hosted conversion and payment acceptance are separate checks; do not create another conversion merely to validate installation.

Client attribution is optional for older callers. CLI 3.2.1 reports its channel and version, with explicit Agent name/version when supplied. The API records the first submission and excludes attribution from the content hash, so upgrading the CLI does not invalidate an uncertain submission retry. Child pages inherit their parent attribution. Older APIs can reject the field; report incompatibility explicitly without dropping attribution and resubmitting.

CLI 3.2.3 sends an explicit `convert --pages` selection in the initial task submission as `pages`. This requires an API that accepts the optional field before installing the new CLI for selected-page conversions. Omitted pages preserve automatic all-page conversion. Explicit selections are normalized and persisted before discovery creates child tasks, removing the race between automatic creation and later selection. Rejection by an older API remains explicit; do not drop the field or retry with a changed request body. Existing tasks continue through status/wait with their original identity.

## R2 inbox (3.3.1)

New submissions use the shared Cloudflare R2 inbox rather than synchronous `/task/create`. See [R2 reception and recovery](submission-inbox.md). The receiving Web release and Native API importer must support optional email, selected pages and task-token hashes before this CLI is distributed. Deploy the independent R2 notification consumer before enabling the creation event rule.
