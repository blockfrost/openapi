const fs = require("fs");
const path = require("path");

const config = JSON.parse(
  fs.readFileSync(path.join(__dirname, "openapi-generator.json"), "utf-8")
);

const cargoPath = path.join(__dirname, "Cargo.toml");
let cargo = fs.readFileSync(cargoPath, "utf-8");

if (config.packageDescription) {
  cargo = cargo.replace(
    /^description = ".*"$/m,
    `description = "${config.packageDescription}"`
  );
}

if (config.packageName) {
  cargo = cargo.replace(
    /^name = ".*"$/m,
    `name = "${config.packageName}"`
  );
}

fs.writeFileSync(cargoPath, cargo);

const stripLeadingBlankLines = dir => {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      stripLeadingBlankLines(full);
    } else if (entry.isFile() && full.endsWith(".rs")) {
      const original = fs.readFileSync(full, "utf-8");
      const stripped = original.replace(/^\s*\n/, "");
      if (stripped !== original) {
        fs.writeFileSync(full, stripped);
      }
    }
  }
};

stripLeadingBlankLines(path.join(__dirname, "src"));

const apisDir = path.join(__dirname, "src", "apis");
if (fs.existsSync(apisDir)) {
  fs.rmSync(apisDir, { recursive: true, force: true });
}

const libPath = path.join(__dirname, "src", "lib.rs");
if (fs.existsSync(libPath)) {
  const lib = fs.readFileSync(libPath, "utf-8");
  const patched = lib.replace(/^pub mod apis;\n?/m, "");
  if (patched !== lib) {
    fs.writeFileSync(libPath, patched);
  }
}
