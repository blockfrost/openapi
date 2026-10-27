import fs from 'fs';
import path from 'path';
import { Kind, parse, print, visit, type ConstDirectiveNode } from 'graphql';

// SpectaQL ignores custom directives, so items marked @beta in the Midnight
// Indexer schema would render like stable API. This writes a docs-only copy
// of the schema with a beta note prepended to each @beta item's description.
// It also drops fields that don't work on Blockfrost yet, listed in HIDDEN.
// The published midnight-indexer-api.graphql stays identical to upstream.

const BETA_NOTE = '**Beta:** this API is unstable and may change without notice.';

// The upstream connection drops (close code 1006) on every bridgeEvents subscription.
const HIDDEN = new Set(['Subscription.bridgeEvents']);

const sourcePath = path.join(__dirname, '../../midnight-indexer-api.graphql');
const targetPath = path.join(
  __dirname,
  '../../.cache/spectaql/midnight-indexer-api.graphql',
);

const isBeta = (directives?: readonly ConstDirectiveNode[]) =>
  directives?.some(directive => directive.name.value === 'beta') ?? false;

const markBeta = <
  T extends {
    directives?: readonly ConstDirectiveNode[];
    description?: { value: string };
  },
>(
  node: T,
): T | undefined => {
  if (!isBeta(node.directives)) return undefined;

  const description = node.description?.value.trim();

  return {
    ...node,
    description: {
      kind: Kind.STRING,
      value: description ? `${BETA_NOTE}\n\n${description}` : BETA_NOTE,
      block: true,
    },
  };
};

const ast = visit(parse(fs.readFileSync(sourcePath, 'utf8')), {
  ObjectTypeDefinition: node => {
    const fields = node.fields?.filter(
      field => !HIDDEN.has(`${node.name.value}.${field.name.value}`),
    );
    const visible = { ...node, fields };

    return markBeta(visible) ?? visible;
  },
  FieldDefinition: markBeta,
});

fs.mkdirSync(path.dirname(targetPath), { recursive: true });
fs.writeFileSync(targetPath, print(ast), 'utf8');
