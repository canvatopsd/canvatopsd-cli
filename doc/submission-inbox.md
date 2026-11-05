# R2 submission reception

The CLI and Web use the same Cloudflare inbox commands: `POST /api/task/prepare` and `POST /api/task/create`. All current task commands use the Web endpoint ending in `/api`. The independent Native API and legacy Ops services have been retired.

A create result with `accepted: true` and `status: "received"` means the original submission and task document were saved to R2 and task payment registration completed. It does not mean conversion started or a preview is ready. `taskId` is the server-issued task ID; `convert` reads the saved task, then returns `awaiting_payment` and the checkout command. After payment, resume conversion progress with `wait TASK` under its deadline. A timeout never cancels the saved request.

The CLI supplies channel `cli`, its version, optional Agent attribution, email, language, layer naming and selected pages. Email is required for all new CLI submissions. A fresh submission gets a fresh 22-character, case-sensitive base64url UUIDv7, even for an unchanged link. Retry an uncertain response with the same URL/options and `--task-id` returned on stderr or in the error.

Task credentials remain private local configuration. The task definition stores only their SHA-256 hash and the Web response sets the task cookie on acknowledgement. Original submissions are retained permanently, independently of execution, payment and delivery status. Web does not persist business fields in browser storage.

R2 object creation is the source of operator notifications. The CLI does not send email or Queue messages. The project's independent Email unit consumes R2 events through its Queue and sends the reminder; repeated event delivery uses a stable mail ID. Existing-task preview, payment and download use the Web product routes and their normal task authorization.

Operator installations use `canvatopsd init --internal` to persist `internalSubmissions: true` in their private configuration. This emits `client.internal: true` in new R2 records so the notification consumer skips them, while still requiring a delivery email address. `deploy:local` enables this for the maintainer installation. Normal npm installations default to external and do not embed any operator identity. Run `init` without `--internal` to configure external mode. This classification does not affect task creation or payment authorization.
