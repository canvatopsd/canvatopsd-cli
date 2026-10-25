import { execFileSync, spawnSync } from "node:child_process";
import {
  chmod,
  mkdir,
  mkdtemp,
  readFile,
  stat,
  writeFile,
} from "node:fs/promises";
import { join } from "node:path";
import { project } from "./paths.mjs";

export async function checkCredentialPrivacy(binary) {
  const root = join(project, ".tmp/artifacts/cli-credential-privacy");
  await mkdir(root, { recursive: true });
  const directory = await mkdtemp(join(root, "native-"));
  const config = join(directory, "config.json");
  const backup = join(directory, "backup.json");
  const task = "11111111-1111-4111-8111-111111111111";
  const token = "a".repeat(64);
  const endpoint = "https://canvatopsd.com/api";
  const env = { ...process.env, CANVATOPSD_CONFIG: config };
  const run = (args, environment = env) =>
    JSON.parse(
      execFileSync(binary, args, { env: environment, encoding: "utf8" }),
    );
  const powershell = (script) =>
    execFileSync(
      "powershell.exe",
      ["-NoProfile", "-NonInteractive", "-Command", script],
      {
        env: { ...env, CANVATOPSD_PRIVACY_DIRECTORY: directory },
        encoding: "utf8",
      },
    );

  await writeFile(
    config,
    JSON.stringify({
      sessions: { [endpoint]: { [`__Host-canvatopsd-task-${task}`]: token } },
    }),
  );
  if (process.platform === "win32") {
    powershell(`
      $ErrorActionPreference = 'Stop'
      $acl = Get-Acl -LiteralPath $env:CANVATOPSD_PRIVACY_DIRECTORY
      $everyone = [System.Security.Principal.SecurityIdentifier]::new('S-1-1-0')
      $rule = [System.Security.AccessControl.FileSystemAccessRule]::new($everyone, 'ReadAndExecute', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
      $acl.AddAccessRule($rule)
      Set-Acl -LiteralPath $env:CANVATOPSD_PRIVACY_DIRECTORY -AclObject $acl
    `);
  } else await chmod(config, 0o666);
  run(["auth", "status"]);
  run(["init"]);
  run(["auth", "export", task, backup]);
  run(["init"], {
    ...env,
    CANVATOPSD_CONFIG: join(directory, "new-state/config.json"),
  });
  if (JSON.parse(await readFile(backup, "utf8")).token !== token)
    throw new Error("Credential export changed the access token");

  if (process.platform === "win32") {
    powershell(`
      $ErrorActionPreference = 'Stop'
      $sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
      foreach ($name in @('config.json', 'config.json.lock', 'backup.json', 'new-state', 'new-state/config.json')) {
        $acl = Get-Acl -LiteralPath (Join-Path $env:CANVATOPSD_PRIVACY_DIRECTORY $name)
        $rules = @($acl.Access)
        if (!$acl.AreAccessRulesProtected -or $rules.Count -ne 1) { throw 'Credential ACL is not private' }
        $rule = $rules[0]
        if ($rule.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value -ne $sid -or $rule.IsInherited -or $rule.AccessControlType -ne 'Allow' -or ($rule.FileSystemRights -band [System.Security.AccessControl.FileSystemRights]::FullControl) -ne [System.Security.AccessControl.FileSystemRights]::FullControl) { throw 'Unexpected credential access rule' }
      }
    `);
  } else {
    for (const name of [
      "config.json",
      "config.json.lock",
      "backup.json",
      "new-state/config.json",
    ])
      if (((await stat(join(directory, name))).mode & 0o777) !== 0o600)
        throw new Error("Credential file permissions are not 0600");
    if (((await stat(join(directory, "new-state"))).mode & 0o777) !== 0o700)
      throw new Error("Credential directory permissions are not 0700");
  }

  for (const args of [
    [`https://canvatopsd.com/task/${task}#access=${token}`],
    ["status", task, `--access=${token}`],
    [
      "convert",
      "https://www.canva.com/design/EXAMPLE/EXAMPLE/view",
      "--seconds",
      token,
    ],
    ["skill", token],
  ]) {
    const result = spawnSync(binary, args, { env, encoding: "utf8" });
    if (
      result.status !== 1 ||
      JSON.parse(result.stdout).error.code !== "invalid_input" ||
      (result.stdout + result.stderr).includes(token)
    )
      throw new Error(
        "Argument diagnostics exposed credentials or lost the error",
      );
  }
  console.log(
    JSON.stringify({
      credentialPrivacy: "verified",
      platform: process.platform,
      argumentCases: 4,
    }),
  );
}
