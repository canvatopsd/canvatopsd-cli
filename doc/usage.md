# CanvaToPSD CLI

The default endpoint is **prod**, `https://canvatopsd.com/api`. The Rust npm launcher requires Node.js 22.16+ and an agent with standard shell access. Business results already use JSON; do not add `--json`.

## Normal workflow

Since CLI 2.3, use the single `taskId` returned by `convert` as the same `TASK` throughout:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
canvatopsd preview TASK --out .tmp/artifacts/preview.webp
canvatopsd checkout TASK
# After the user pays:
canvatopsd download TASK --out .tmp/artifacts/design.psd
```

`convert` submits the design, discovers pages and waits for previews. A single page is handled automatically. For multiple pages, select explicitly with `--pages 1,3`; the CLI never guesses. Later operations use design page numbers, without requiring internal conversion IDs:

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL' --pages 1,3
canvatopsd preview TASK --page 3 --out .tmp/artifacts/page3.webp
canvatopsd checkout TASK --page 3
canvatopsd download TASK --page 3 --out .tmp/artifacts/page3.psd
```

`TASK` remains the same task ID. `--page` is a 1-based design page number, not the file manifest's pageId. Use the advanced `--artifact-page ID` option for preview/download only when an internal file page is needed. When several pages are selected and no page number is supplied, the CLI reports an error listing the available page numbers.

## Waiting, status and recovery

The `convert --seconds 300` deadline covers discovery, page-selection submission, status requests and polling. The initial creation request separately uses the global `--timeout`. A timeout does not cancel conversion; the error includes `taskId` and `submitKey`. Resume with `wait TASK`. Use `status TASK` to inspect selected page numbers, each page's status and required input. A multi-page task becomes ready only when all selected pages are ready.

A multi-page design without `--pages` returns `page_selection_required` and preserves the task. Run `select TASK 1,3`, then `wait TASK`. Existing tasks do not require another convert.

Convert writes its initial submission key to stderr before the request and includes it in the result or error. After an uncertain response, retry with the original URL, page selection and `--key UUID`. Page selection uses the task ID as a stable idempotency key. A request that conflicts with an already confirmed selection fails explicitly instead of creating another set of pages. To resume waiting alone, use wait without resubmitting.

## Common commands

| Command                          | Purpose                                                                                   |
| -------------------------------- | ----------------------------------------------------------------------------------------- |
| `convert URL [--pages 1,3]`      | Submit and wait for previews; return the task ID used afterward                           |
| `status TASK` / `wait TASK`      | Inspect selected pages / wait within a bounded deadline                                   |
| `preview TASK --out FILE`        | Save a preview                                                                            |
| `checkout TASK`                  | Create a checkout link for the user to pay                                                |
| `download TASK --out FILE`       | Synchronize payment and download; verify size and SHA-256                                 |
| `pages TASK` / `select TASK 1,3` | Inspect and select pages in a separate-step workflow                                      |
| `payment TASK` / `confirm TASK`  | Synchronize payment separately / confirm an authorized payment; confirm may capture funds |
| `files TASK`                     | List paid files                                                                           |
| `create URL`                     | Submit only, without discovery or waiting                                                 |
| `doctor`                         | Check API routing and the task-credential protocol                                        |

`preview`, `checkout`, `download`, `files`, `payment` and `confirm` support `--page NUMBER`. The normal workflow does not require a separate payment command.

`checkoutUrl` is a CanvaToPSD-hosted checkout entry. Give it to the user unchanged; do not substitute an old PayPal URL. It establishes a task cookie in the browser that opens it, then continues to the same PayPal order. This also works for existing orders and does not require opening a private task link first. The entry lasts seven days. Run checkout again to obtain a fresh entry without reconverting or creating another payment order. The API unit maintains the protocol in `doc/task-return-routing.md`.

`convert`, `create` and `select` with `--dry-run` validate locally without saving credentials or submitting a task. `checkout --dry-run` reads the live quote without creating a payment. Business status is fetched each time; the CLI does not save a latest task or cache quotes.

Downloads default to `page-0001.psd`. Use `--file fonts.zip` or `--file fonts.txt` for fonts, and `--file preview-layers.json` / `preview-layers.webp` for layer previews. Existing destination files cause an explicit error. A preview does not establish payment, and a successful download does not establish native Photoshop opening or semantic editability.

## Task credentials and optional email

No account, API key or email is required. For each new submission, the CLI generates a separate random token and saves it in its local mode-0600 configuration before sending the request. The API stores only a hash and returns the task cookie. The token is never printed. A retry with `--key UUID` reuses the saved token. `auth status` lists saved submission keys. A submission key alone cannot issue replacement access credentials.

`--email EMAIL` enables email notifications. Without it, creation, conversion readiness, manual-input requests and payment produce no notification emails. Inspect required input through status/wait. Original credentials remain valid after readiness and payment; parent credentials authorize direct child tasks. The CLI sends only the target or parent credentials. Responses to select, pages and parent status save authorized relationships. For child tasks created by an older version, query parent status once to restore the relationship.

To back up credentials or move machines:

```sh
canvatopsd auth export TASK .tmp/artifacts/task-access.json
# On another machine using the same service environment:
canvatopsd auth import /absolute/task-access.json
```

The export contains one task's secret credentials, uses mode 0600 and refuses overwrites. Do not upload it to Git or paste it into chat. When a child uses parent authorization, export the parent. Select the matching service endpoint before importing. Existing files containing emailed private links can still be imported, including language prefixes such as `zh-tw` and `pt-br`. `auth clear` removes cookies, retry tokens and parent/child relationships for the selected endpoint. Without email, losing both local credentials and backups prevents self-service access recovery.

Payment remains task-based; an API key or email is not a payment requirement. Query `payment TASK` after the user approves payment. Run `confirm TASK` only with authorization to confirm payment. Payment webhooks can update status; existing task credentials continue to authorize downloads.

This flow requires API support for taskToken and optional email. An older API rejects the request explicitly. The CLI still defaults to prod; API prod releases are separate.

## Installation and Skill

The npm package is `@canvatopsd/cli`; the executable is `canvatopsd`. Install the currently deployed official download with:

```sh
npm install -g https://canvatopsd.com/agent/cli.tgz
canvatopsd --version
canvatopsd skill install codex
```

The website package follows the Web deployment version. Once the scoped package is publicly published to npm, package-name installation and npx become available:

```sh
npx --yes @canvatopsd/cli --help
npm install -g @canvatopsd/cli
```

For local installation from standalone source, run at the repository root:

```sh
npm ci --cache .tmp/npm-cache
npm run check
npm pack
npm run deploy:local
canvatopsd skill install codex
```

In the product monorepo, use corresponding `npm --prefix cli ...` commands, such as `npm --prefix cli run check`, and explicitly put npm cache in the monorepo root `.tmp/npm-cache`. Build tools find the actual Git root, so each checkout keeps temporary files in its own root `.tmp/`.

Deployment installs the checked archive globally through npm; `npm prefix -g` determines its location. Run npm pack first, then deploy:local. Deployment unpacks and checks the actual archive before installation. It updates only the CLI, without releasing Web/API/Runner. The hosted protocol is implemented in Rust; no private API source checkout is required.

Skill targets `codex`, `claude-code`, `cursor`, `copilot` and `agents` use `.codex/skills`, `.claude/skills`, `.cursor/skills`, `.copilot/skills` and `.agents/skills` under the user directory. The default is codex. Use `canvatopsd skill install --dir /path/to/project/.agents/skills` for a project directory. Existing Skills cause an explicit error rather than overwriting manual edits.

Skill source is [skills/canvatopsd/SKILL.md](../skills/canvatopsd/SKILL.md). Any agent supporting generic Agent Skills and shell access can use it. If automatic discovery is unavailable, ask the agent to read that file.

To distribute an installation package without npm login, run from the CLI repository root, or `cli/` in the product monorepo:

```sh
npm pack
npm install -g ./releases/canvatopsd-cli-VERSION.tgz
```

Replace VERSION with the actual version. The package includes the README, usage guide and complete Skill; runtime execution does not download dependencies or access the source repository. `.npmrc` directs package output to `releases/canvatopsd-cli-VERSION.tgz`. Formal archives are tracked in Git and preserved by version. Never overwrite an existing version; run `npm version patch` or explicitly select a new version before repackaging. Build caches and acceptance evidence stay in the untracked root `.tmp/`. Packaged `release.json` records version, source commit, build time and native targets. Packages do not recursively include releases/.

Maintainers publish the exact checked archive using `npm run release:npm`.
`npm run release:npm -- --dry-run` verifies publication readiness without publishing.
All six targets need native acceptance receipts matching the source revision,
version and binary digest. See the [source release checklist](https://github.com/canvatopsd/canvatopsd-cli/blob/main/doc/releasing.md).
Publishing requires access to the npm scope and a fresh immutable version.
If packaging is interrupted, run `npm run postpack` before retrying.

## Configuration and advanced commands

Prod works without initialization. `canvatopsd init` saves prod; `canvatopsd init http://127.0.0.1:8080/api` explicitly selects local. Use global `--base-url URL` for a temporary endpoint. `--origin` is only for the local proxy's Origin check. `--timeout` is the per-request timeout in seconds, default 30.

Configuration precedence is arguments > environment > configuration file > defaults. `CANVATOPSD_BASE_URL` and `CANVATOPSD_ORIGIN` supply connection settings. `CANVATOPSD_CONFIG` defaults to `~/.config/canvatopsd/config.json`. It stores connection settings, cookies and retry tokens, with mode 0600. Updates use a file lock, reread the latest content inside the lock and merge only the current changes. Contention returns ELOCKED explicitly; retry after the other command finishes. Uncertain creation still reuses --key. Multiple agents can isolate credentials with CANVATOPSD_CONFIG.

Normal Web calls require no operator token. Direct API operator queries use CANVATOPSD_OPERATOR_TOKEN; direct API customer commands also require CANVATOPSD_PROXY_TOKEN. Operator identity does not replace task cookies. HTTPS uses both Node and system trust stores without disabling certificate validation.

```sh
canvatopsd resolve 'CANVA_URL'
canvatopsd --base-url http://127.0.0.1:8797 operator task-list --offset 0
canvatopsd request /task/status --body .tmp/request.json
```

`operator notification-status TASK` reads notification records. Task lists use pages of 100. Request always uses POST; --operator adds operator authorization, and --out FILE saves binary data. The API provider maintains request structures under api/contracts/; the standalone CLI uses those definitions through the pinned @canvatopsd/client-contracts dependency. Raw commands can change business data and are not read-only diagnostics.

## Output and errors

Success writes `{"ok":true,"data":...}` to stdout and exits 0. Failure writes `{"ok":false,"error":{"code":"...","message":"..."}}` and exits 1. API errors also include HTTP status; submission errors include submitKey. Help and version use text. Stderr carries diagnostics such as submission keys.

For 401/403, restore credentials from a matching-environment backup or emailed private link; the file must include the complete `/task/UUID#access=...` address. Do not print private links or configuration contents. For 409, inspect task status and handle its code. After wait_timeout, continue querying; the background task is not canceled. Doctor uses read-only `POST /task/protocol/read` to check actual API routing, database access, taskToken and optional-email support. An unsupported API or unsynchronized Web route fails explicitly. Passing doctor does not validate a task's credentials, Runner, payment or delivery.

Version 2.0 replaced task create/status/wait, page list/request, preview list/download and delivery list/download with the shorter commands above. Legacy arguments have no compatibility layer.

## Agent discovery and self-service

```sh
canvatopsd --help
canvatopsd create --help
canvatopsd discover
canvatopsd discover 'skill install'
canvatopsd skill show
canvatopsd skill status codex
# After reviewing and authorizing replacement of the installed Skill:
canvatopsd skill install codex --replace
```

`discover [command]` returns an offline JSON command tree. Arguments, options, required fields and defaults come directly from actual command definitions, with examples and side-effect guidance. Quote nested command paths. Discovery reads no credentials, connects to no service and submits no task. Replace example placeholders before execution.

Skill show returns the bundled entry document as JSON. Skill status compares installed and bundled files and reports missing or changed files. Current only means file equality; it does not establish that an agent loaded the Skill, PATH is correct or the service is healthy. A successful command in the target agent's terminal establishes that terminal can find the CLI. Start a new agent session after installation or updates to refresh discovery.

Skill install --replace explicitly replaces the entire target Skill directory, including manual edits. The default refuses overwrites. Installation supports codex, claude-code, cursor, copilot, agents or a custom --dir. Codex uses CODEX_HOME/skills when configured. SKILL.md contains task selection, the shortest workflow and delivery conditions; credential recovery and advanced operations are loaded from references/operations.md as needed.

A project can add this to its existing agent instructions: “Use the canvatopsd Skill for Canva to PSD conversion, task previews and downloads. Run canvatopsd discover for the command index.” This repository's entry is skills/canvatopsd/SKILL.md. No MCP server is needed for shell-based use; remote or shell-free integrations require separate evaluation.
