# CLI and Skill access audit

CLI 3.2.0 and its bundled Skill were reviewed on 2026-10-27 around a stable product interface: task identity, design page numbers, Web preview, checkout and verified downloads.

## Product boundaries

- `preview TASK` returns a server-issued private `previewUrl`, with optional `--page N`. The Web interface owns task entry, layer rendering and preview resource formats. `--out FILE` saves only a whole-page image. The CLI does not construct Web routes or use checkout to obtain preview access.
- Preview links can be imported from a file with `auth import FILE`. Existing task IDs, credentials and submission keys remain usable.
- Downloads use `--format psd|fonts|font-list`. Internal preview-file and artifact-page options are removed. File listing, page discovery, selection, checkout and payment results expose product fields instead of internal identifiers and raw manifests.
- Status identifies the page requiring input and includes its message, so the caller can continue using the original task rather than searching for a child conversion.
- Default discovery includes ordinary product commands. `discover --advanced` and explicit command discovery retain maintainer access. Raw requests are not a recovery step in the Skill.
- The Skill and user documentation explain product actions and recovery. Credentials, browser cookies, API routes, layer JSON and atlas handling are not part of the normal Agent workflow.

## Compatibility and release ordering

This minor release changes preview results and removes internal artifact flags. Scripts using those details must move to Web preview links, design page numbers and download formats. The API preview-link commands and the Web entry handler must be released before public CLI distribution. Missing support fails explicitly; doctor checks only the base protocol. Local installation, public npm and website distribution remain separate releases.

## Validation

Source validation covers rustfmt, Clippy with warnings denied, JavaScript syntax, Markdown formatting, API lint/type checks, Web type checks and Skill validation. Each deployment unit checks its final build before local deployment. CLI packaging additionally checks extracted archives, native identity, privacy, unresolved LFS pointers, version consistency and generated discovery documentation.

An isolated D1 and the final API Worker exercised link issuance and browser access, parent authorization, page validation, tampering, expiry, purpose mismatch, task mismatch and revoked access. No preview operation made a payment or outbound provider call. The final Web browser entry established access before loading the selected page, cleared its fragment, survived refresh and stored no business fields in browser storage.

Temporary CLI integration checks cover stable result fields, PSD/font formats, image saving, page selection, required-input messages, payment reads, download integrity and overwrite refusal. Synthetic file-transfer checks do not establish Photoshop compatibility. No new conversion jobs, real payments, emails, public npm publication or prod deployment were part of this audit. Local native acceptance is macOS ARM64; other platforms require their own receipts.

## Submission attribution in 3.2.1

CLI submissions report their own version automatically. Agents pass `--agent NAME` and, when known, `--agent-version VERSION`. Attribution is explicit, retained from the first submission and inherited by child pages. It is excluded from the API content hash so a CLI update does not invalidate uncertain-submission recovery. Missing historical attribution stays unknown. The Skill does not guess Agent versions or send a model version as an application version.
