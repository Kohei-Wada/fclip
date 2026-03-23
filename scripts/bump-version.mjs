import { readFileSync, writeFileSync } from "fs";
import { execSync } from "child_process";

const version = process.argv[2];
if (!version || !/^\d+\.\d+\.\d+$/.test(version)) {
  console.error("Usage: npm run bump -- <version> (e.g. 0.3.0)");
  process.exit(1);
}

function replaceVersion(file, pattern, replacement) {
  const content = readFileSync(file, "utf-8");
  const updated = content.replace(pattern, replacement);
  if (content === updated) {
    console.warn(`  WARNING: no change in ${file}`);
  }
  writeFileSync(file, updated);
}

console.log(`Bumping version to ${version}...`);

// package.json
replaceVersion("package.json", /"version": "\d+\.\d+\.\d+"/, `"version": "${version}"`);

// tauri.conf.json
replaceVersion(
  "src-tauri/tauri.conf.json",
  /"version": "\d+\.\d+\.\d+"/,
  `"version": "${version}"`,
);

// Cargo.toml (only the first occurrence under [package])
replaceVersion(
  "src-tauri/Cargo.toml",
  /^version = "\d+\.\d+\.\d+"/m,
  `version = "${version}"`,
);

// Update lock files
console.log("Updating lock files...");
execSync("npm install --package-lock-only --silent", { stdio: "inherit" });
execSync("cargo check --quiet --manifest-path src-tauri/Cargo.toml", { stdio: "inherit" });

console.log(`\nDone! Updated to v${version}`);
console.log("  - package.json");
console.log("  - src-tauri/Cargo.toml");
console.log("  - src-tauri/tauri.conf.json");
console.log("  - package-lock.json (auto)");
console.log("  - Cargo.lock (auto)");
