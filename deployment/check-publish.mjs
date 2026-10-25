import { archive, checkNpmArchive } from "./check-npm-archive.mjs";
import { checkRelease } from "./check-release.mjs";
// npm invokes prepublishOnly before prepack. Only a complete checked archive may be published.
const directory = await checkNpmArchive(archive);
await checkRelease(directory, { publishing: true });
