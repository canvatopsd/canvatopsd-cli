import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";

const forbidden = [
  ["developer home path", /\/(?:Users|home)\/[^/\s\x00"<>]+\//],
  [
    "Windows developer home path",
    /[A-Za-z]:[\\/]Users[\\/][^\\/\s\x00"<>]+[\\/]/,
  ],
  ["internal source repository host", /git\.code\.tencent\.com/i],
  ["local service hostname", /\blocal\.[a-z0-9-]+\.[a-z]{2,}\b/i],
  [
    "private key block",
    /-----BEGIN (?:RSA |EC |OPENSSH |DSA |ENCRYPTED )?PRIVATE KEY-----\s+[A-Za-z0-9+/=\r\n]{64,}\s+-----END (?:RSA |EC |OPENSSH |DSA |ENCRYPTED )?PRIVATE KEY-----/,
  ],
];
export async function checkPrivacy(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) await checkPrivacy(path);
    else if (entry.isFile()) {
      const content = (await readFile(path)).toString("latin1");
      for (const [reason, pattern] of forbidden) {
        if (pattern.test(content))
          throw new Error(`Release privacy check failed (${reason}): ${path}`);
      }
    } else throw new Error(`Unexpected release entry: ${path}`);
  }
}
