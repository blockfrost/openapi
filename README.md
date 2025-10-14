<img src="https://blockfrost.io/images/logo.svg" width="250" align="right" height="90" style="margin-bottom: -50px">

# Blockfrost.io OpenAPI

<br>
<p align="center">Open Source OpenAPI specification for <a href="https://blockfrost.io">Blockfrost.io</a> backend API.</p>

<div align="center">

![GitHub](https://img.shields.io/github/license/blockfrost/openapi)
![master build ci](https://github.com/blockfrost/openapi/actions/workflows/CI.yaml/badge.svg?branch=master)
[![npm version](https://badge.fury.io/js/%40blockfrost%2Fopenapi.svg)](https://badge.fury.io/js/%40blockfrost%2Fopenapi)
![downloads](https://img.shields.io/npm/dy/@blockfrost/openapi)
<a href="https://fivebinaries.com/"><img src="https://img.shields.io/badge/made%20by-Five%20Binaries-darkviolet.svg?style=flat-square" /></a>

</div>
<p align="center">
  <a href="#getting-started">Getting started</a> •
  <a href="#development">Development</a>
</p>

## Getting started

Active version can be found in `released` branch and also in [GitHub releases](https://github.com/blockfrost/openapi/releases).

Development version is in `master` branch and is being merged into `released` upon each release.

Released documentation can be found at [docs.blockfrost.io](https://docs.blockfrost.io/).

## Development

Blockfrost OpenAPI [`blockfrost-openapi.yaml`](blockfrost-openapi.yaml) specification is generated from all yaml files in `src` directory.
Then there is Mithril Aggregator API spec [`mithril.yaml`](mithril.yaml) which can be downloaded from [Mithril Github](https://github.com/input-output-hk/mithril).
These two specs are then merged together via `openapi-merge-cli` (configuration is inside [`openapi-merge.json`](openapi-merge.json)).
Only the Mithril endpoints with a tag `Cardano » Mithril` are included into the final spec.

> Tag `Cardano » Mithril` needs to be added manually to each relevant endpoint in Mithril OpenAPI spec.

If you add a new file then don't forget to add it to `paths` in [`src/definitions.yaml`](src/definitions.yaml).

Edit the source yaml files and build the package:

```typescript
yarn build
```

Feel free to open PR against the `master` branch. It is a great place to start any discussion for new features and changes to the Blockfrost API.

### UI

When you push a new commit, the documentation for your branch is automatically generated on Vercel and added to your PR as a deployment.

## Usage

You can download [`openapi.yaml`](openapi.yaml) directly from the repository or use this project as a dependency in your JavaScript/TypeScript project.

### Typescript example

Install `@blockfrost/openapi`:

```console
yarn add @blockfrost/openapi
```

or

```console
npm install @blockfrost/openapi
```

Now you can use TypeScript types generated from the OpenAPI specification:

```typescript
import { components } from '@blockfrost/openapi';

type Block = components['schemas']['block_content'];
type Address = components['schemas']['address_content'];
type UtxoAsset = components['schemas']['address_utxo_content'];
```
