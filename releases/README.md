# Versioned CLI packages

The maintained source checkout stores immutable npm archives here as `canvatopsd-cli-VERSION.tgz`. Existing versions must never be overwritten. See [development and release instructions](../doc/development.md).

Version 3.3.3 is published on npm with accepted binaries for macOS ARM64, Linux ARM64/x64 and Windows ARM64/x64. The retained archive is the exact registry download, verified against its SHA-512 integrity and native platform receipts. Install it with:

```sh
npm install -g @canvatopsd/cli@3.3.3
```

The independently deployed website download is available with:

```sh
npm install -g https://canvatopsd.com/agent/cli.tgz
```

The website package version follows the website deployment. Publishing `@canvatopsd/cli` on the public npm registry is a separate release step; a GitHub repository does not make the npm package public.
