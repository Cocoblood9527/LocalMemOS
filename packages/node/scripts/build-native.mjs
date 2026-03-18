import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRoot = resolve(__dirname, "../../..");

const result = spawnSync("cargo", ["build", "-p", "memory-node"], {
  cwd: repoRoot,
  stdio: "inherit",
});
if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

const libName = process.platform === "darwin"
  ? "libmemory_node.dylib"
  : process.platform === "win32"
    ? "memory_node.dll"
    : "libmemory_node.so";

const src = join(repoRoot, "target", "debug", libName);
const dest = join(repoRoot, "packages", "node", "memory-node.node");

if (!existsSync(src)) {
  console.error(`native library not found: ${src}`);
  process.exit(1);
}

mkdirSync(dirname(dest), { recursive: true });
copyFileSync(src, dest);
