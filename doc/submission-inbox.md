# R2 submission reception

CLI 3.3.1 and Web use the same Cloudflare inbox commands: `POST /api/submission/prepare` and `POST /api/submission/create`. New submissions require a Web endpoint ending in `/api`; direct API endpoints remain available for existing-task and operator commands.

A create result with `accepted: true` and `status: "received"` means the immutable record and task-import marker were saved to R2. It does not mean conversion started or a preview is ready. `taskId` equals the server-issued `submissionId`; the returned `submitKey` is the same value. `convert` subsequently waits for task import and conversion under the existing deadline. A timeout never cancels the saved request.

The CLI supplies channel `cli`, its version, optional Agent attribution, email, language, layer naming and selected pages. Email remains optional. A fresh submission gets a fresh UUIDv7, even for an unchanged link. Retry an uncertain response with the same URL/options and `--key` returned on stderr or in the error. UUIDv4 keys from older synchronous submissions must be recovered with existing-task commands rather than replayed as new inbox requests.

Task credentials remain private local configuration. The inbox stores only their SHA-256 hash, sets the task cookie on acknowledgement, and the importer creates the matching task grant. An authenticated status request can report that the request is still waiting for import without requiring the product API. Web does not persist business fields in browser storage.

R2 object creation is the source of operator notifications. Neither CLI nor Web depends on email services or sends Queue messages. The independent Cloudflare notification consumer reads the original record and sends the reminder; repeated event delivery uses a stable mail ID. Existing-task preview, payment and download still require the product API and their normal authorization.

Operator installations use `canvatopsd init --internal` to persist `internalSubmissions: true` in their private configuration. This emits `client.internal: true` in new R2 records so the notification consumer skips them, including when no email is provided. `deploy:local` enables this for the maintainer installation. Normal npm installations default to external and do not embed any operator identity. Run `init` without `--internal` to configure external mode. This classification does not affect task creation or payment authorization.
