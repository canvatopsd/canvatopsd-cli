# Versioned CLI packages

The maintained source checkout stores immutable npm archives here as `canvatopsd-cli-VERSION.tgz`. Existing versions must never be overwritten. See [development and release instructions](../doc/development.md).

Version 3.4.16 is published on npm with accepted binaries for macOS ARM64, Linux ARM64/x64 and Windows ARM64/x64. The retained archive is the exact registry download, verified against its SHA-512 integrity and native platform receipts. Install it with:

```sh
npm install -g @canvatopsd/cli@3.4.16
```

macOS ARM64 native acceptance for 3.4.16 ran on an Apple Silicon Mac. Linux and Windows native acceptance ran on GitHub-hosted runners. The aggregate release verified all five source-matched native receipts.

The independently deployed website provides the [Agent setup page](https://canvatopsd.com/ai-agent), public Skill and matching command documentation. Install native CLI packages through npm.

The website documentation version follows the Web deployment. Publishing `@canvatopsd/cli` on the public npm registry is a separate release step; a GitHub repository does not make the npm package public.
