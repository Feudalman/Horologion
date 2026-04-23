import { copyFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const targetTriple = getTargetTriple();
const executableName = process.platform === "win32" ? "listener.exe" : "listener";
const sidecarName =
  process.platform === "win32"
    ? `listener-${targetTriple}.exe`
    : `listener-${targetTriple}`;

console.log(`Building listener sidecar for ${targetTriple}...`);

const cargo = spawnSync(
  "cargo",
  ["build", "-p", "listener", "--release", "--target", targetTriple],
  {
    cwd: process.cwd(),
    stdio: "inherit",
  },
);

if (cargo.status !== 0) {
  process.exit(cargo.status ?? 1);
}

const sourcePath = join(
  process.cwd(),
  "target",
  targetTriple,
  "release",
  executableName,
);
const binDir = join(process.cwd(), "src-tauri", "bin");
const targetPath = join(binDir, sidecarName);

mkdirSync(binDir, { recursive: true });
copyFileSync(sourcePath, targetPath);

console.log(`Copied listener sidecar to ${targetPath}`);

function getTargetTriple() {
  if (process.env.TAURI_ENV_TARGET_TRIPLE) {
    return process.env.TAURI_ENV_TARGET_TRIPLE;
  }

  if (process.env.TARGET) {
    return process.env.TARGET;
  }

  if (process.platform === "darwin") {
    return process.arch === "arm64"
      ? "aarch64-apple-darwin"
      : "x86_64-apple-darwin";
  }

  if (process.platform === "win32") {
    return process.arch === "arm64"
      ? "aarch64-pc-windows-msvc"
      : "x86_64-pc-windows-msvc";
  }

  if (process.platform === "linux") {
    return process.arch === "arm64"
      ? "aarch64-unknown-linux-gnu"
      : "x86_64-unknown-linux-gnu";
  }

  throw new Error(`Unsupported platform: ${process.platform}/${process.arch}`);
}
