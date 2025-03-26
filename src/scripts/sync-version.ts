import fs from 'fs';
import path from 'path';

const packageJson = JSON.parse(
  fs.readFileSync(path.join(__dirname, '../../package.json'), 'utf8'),
);

const newVersion = packageJson.version;
const openapiContent = fs.readFileSync(
  path.join(__dirname, '../definitions.yaml'),
  'utf8',
);

const updatedContent = openapiContent.replace(
  /^(\s*version:\s*).+$/m,
  `$1${newVersion}`,
);

fs.writeFileSync(
  path.join(__dirname, '../definitions.yaml'),
  updatedContent,
  'utf8',
);
