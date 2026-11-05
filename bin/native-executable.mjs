export function nativeExecutable(version, environment, target) {
  return `canvatopsd-cli-v${version}-${environment}${target.startsWith("win32-") ? ".exe" : ""}`;
}
