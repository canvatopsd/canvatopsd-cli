# Submission compatibility

CLI 3.1.5 omits the fixed `locale: "en"` field when creating a task. The CLI has no locale option, and the API already owns the default language. Sending this optional field previously made otherwise valid submissions fail against the deployed strict command schema with `command_invalid`.

This is a request construction fix: it does not retry invalid commands or weaken API validation. An uncertain submission continues to use its original submission key and saved task credential.

Native platform acceptance covers offline command discovery, submission dry-run and credential privacy. Hosted conversion and payment acceptance are separate checks; do not create another conversion merely to validate installation.
