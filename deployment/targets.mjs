export const targets = {
  "darwin-arm64": { rust: "aarch64-apple-darwin", executable: "canvatopsd" },
  "linux-arm64": {
    rust: "aarch64-unknown-linux-gnu",
    executable: "canvatopsd",
  },
  "linux-x64": { rust: "x86_64-unknown-linux-gnu", executable: "canvatopsd" },
  "win32-arm64": {
    rust: "aarch64-pc-windows-msvc",
    executable: "canvatopsd.exe",
  },
  "win32-x64": { rust: "x86_64-pc-windows-msvc", executable: "canvatopsd.exe" },
};
export const hostTarget = `${process.platform}-${process.arch}`;
