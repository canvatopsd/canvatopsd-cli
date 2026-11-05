import { execFileSync, spawnSync } from "node:child_process";
import {
  chmod,
  mkdir,
  mkdtemp,
  readFile,
  stat,
  writeFile,
} from "node:fs/promises";
import { dirname, join, relative } from "node:path";
import { project } from "./paths.mjs";

export async function checkCredentialPrivacy(binary) {
  const root = join(project, ".tmp/artifacts/cli-credential-privacy");
  await mkdir(root, { recursive: true });
  const directory = await mkdtemp(join(root, "native-"));
  const env = { ...process.env, HOME: directory, USERPROFILE: directory };
  const installation = JSON.parse(
    execFileSync(binary, ["installation-info"], { env, encoding: "utf8" }),
  );
  const state = join(
    ".local/share/canvatopsd",
    installation.environment,
    "data/cli/session.json",
  );
  const config = join(directory, state);
  const freshHome = join(directory, "new-state");
  const freshConfig = join(freshHome, state);
  const backup = join(directory, "backup.json");
  const task = "AZpQWQAAcACAAAAAAAAAAA";
  const token = "a".repeat(64);
  const endpoint = "https://canvatopsd.com/api";

  const run = (args, environment = env) =>
    JSON.parse(
      execFileSync(binary, args, { env: environment, encoding: "utf8" }),
    );
  const powershell = (script) =>
    execFileSync(
      "pwsh",
      ["-NoProfile", "-NonInteractive", "-Command", script],
      {
        env: { ...env, CANVATOPSD_PRIVACY_DIRECTORY: directory },
        encoding: "utf8",
      },
    );

  await mkdir(dirname(config), { recursive: true });
  await mkdir(freshHome);
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
    HOME: freshHome,
    USERPROFILE: freshHome,
  });
  if (JSON.parse(await readFile(backup, "utf8")).token !== token)
    throw new Error("Credential export changed the access token");

  const credentialFiles = [config, `${config}.lock`, backup, freshConfig];
  const credentialDirectory = dirname(freshConfig);
  if (process.platform === "win32") {
    const names = [...credentialFiles, credentialDirectory]
      .map((path) => `'${relative(directory, path).replaceAll("'", "''")}'`)
      .join(", ");
    powershell(`
      $ErrorActionPreference = 'Stop'
      $sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
      foreach ($name in @(${names})) {
        $acl = Get-Acl -LiteralPath (Join-Path $env:CANVATOPSD_PRIVACY_DIRECTORY $name)
        $rules = @($acl.Access)
        if (!$acl.AreAccessRulesProtected -or $rules.Count -ne 1) { throw 'Credential ACL is not private' }
        $rule = $rules[0]
        if ($rule.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value -ne $sid -or $rule.IsInherited -or $rule.AccessControlType -ne 'Allow' -or ($rule.FileSystemRights -band [System.Security.AccessControl.FileSystemRights]::FullControl) -ne [System.Security.AccessControl.FileSystemRights]::FullControl) { throw 'Unexpected credential access rule' }
      }
    `);
  } else {
    for (const path of credentialFiles)
      if (((await stat(path)).mode & 0o777) !== 0o600)
        throw new Error("Credential file permissions are not 0600");
    if (((await stat(credentialDirectory)).mode & 0o777) !== 0o700)
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
