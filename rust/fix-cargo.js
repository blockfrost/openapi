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
