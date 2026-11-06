# Operation analytics

CLI 3.4.15 adds one anonymous completion event for online commands through the
configured Web endpoint. It includes the native version, command name,
success/error result, elapsed time, OS/CPU and applicable page/format options.
The service forwards events to Google Analytics with `client_type=cli`;
browser interactions use `client_type=web` and the Web application's version.

No email, Canva URL, task credential, task ID, local file path, raw command line
or request body is included. Each command uses a fresh random ID, not a person
or persistent installation identity. Polling does not multiply command counts.

Help/version, parse errors, offline commands (`discover`, `skill`, `resolve`,
`init`, `auth`) and dry-runs do not send analytics. Online commands include
update checks and advanced requests. Local records payloads without contacting
Google. Separately configured non-Web API endpoints do not receive analytics.

Collection is attempted once with a three-second timeout after the command
returns. An `analytics_record_failed` stderr warning reports an unacknowledged
collection. The original business JSON and exit status remain unchanged. No
business action or uncertain analytics send is replayed.

Opening checkout or preview in a browser produces Web activity independently.
Copying a command on the website is Web activity, not proof the CLI ran.
