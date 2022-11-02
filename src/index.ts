import fs from 'fs';
import path from 'path';
import YAML from 'yaml';

const file = fs.readFileSync(
  path.resolve(__dirname, '../openapi.yaml'),
  'utf8',
);
const spec = YAML.parse(file);

export const getSchemaForEndpoint = (endpointName: string) => {
  if (!spec.paths[endpointName]) {
    throw Error(
      `Missing Blockfrost OpenAPI schema for endpoint "${endpointName}".`,
    );
  }

  const responses: any = { response: {} };
  for (const response of Object.keys(spec.paths[endpointName].get.responses)) {
    // success 200
    if (response === '200') {
      const referenceOrValue =
        spec.paths[endpointName].get.responses['200'].content[
          'application/json'
        ].schema;

      // is reference -> resolve references
      if (referenceOrValue['$ref']) {
        const schemaName = referenceOrValue['$ref'].replace(
          '#/components/schemas/',
          '',
        );
        const schemaReferenceOrValue = spec.components.schemas[schemaName];

        // is nested reference
        if (
          schemaReferenceOrValue.items &&
          schemaReferenceOrValue.items['$ref']
        ) {
          const nestedSchemaName = schemaReferenceOrValue.items['$ref'].replace(
            '#/components/schemas/',
            '',
          );

          if (schemaReferenceOrValue.type) {
            responses.response[200] = {
              ...schemaReferenceOrValue,
              items: spec.components.schemas[nestedSchemaName],
            };
          } else {
            responses.response[200] = spec.components.schemas[nestedSchemaName];
          }
        } else {
          // is not nested reference
          responses.response[200] = spec.components.schemas[schemaName];
        }
      } else {
        // is not reference
        responses.response[200] = referenceOrValue;
      }

      // anyOf case
      if (referenceOrValue['anyOf']) {
        const anyOfResult: any = { anyOf: [] };

        for (const anyOfItem of referenceOrValue['anyOf']) {
          const schemaName = anyOfItem['$ref'].replace(
            '#/components/schemas/',
            '',
          );

          anyOfResult['anyOf'].push(spec.components.schemas[schemaName]);
        }

        responses.response[200] = anyOfResult;
      }

      const parameters = spec.paths[endpointName].get.parameters;

      if (parameters) {
        const queryParams = parameters.filter((i: any) => i.in === 'query');
        let queryProps: any = {};

        if (queryParams && queryParams.length > 0) {
          queryParams.forEach((param: any) => {
            queryProps[param.name] = param.schema;
          });

          responses['querystring'] = {
            type: 'object',
            properties: queryProps,
          };
        }

        const pathparams = parameters.filter((i: any) => i.in === 'path');

        if (pathparams && pathparams.length > 0) {
          let pathProps: any = {};

          pathparams.forEach((param: any) => {
            pathProps[param.name] = param.schema;
          });

          responses['params'] = {
            type: 'object',
            properties: pathProps,
          };
        }

        // const query = parameters.filter((i: any) => i.in === 'param');
        // let queryParams: any = {};
      }

      // 1 bug -> edge case
      if (endpointName === '/txs/{hash}/metadata') {
        responses.response[200] = {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              label: {
                type: 'string',
              },
              json_metadata: {
                // possible bug FIXME https://github.com/fastify/fast-json-stringify/issues/246
                // oneOf: [
                //   {
                //     type: 'string',
                //   },
                //   {
                //     type: 'object',
                //   },
                // ],
              },
            },
            required: ['label', 'json_metadata'],
          },
        };
      }

      // 2 bug -> edge case
      if (endpointName === '/nutlink/{address}/tickers/{ticker}') {
        responses.response[200] = {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              tx_hash: {
                type: 'string',
              },
              block_height: {
                type: 'integer',
              },
              tx_index: {
                type: 'integer',
              },
              payload: {
                // possible bug FIXME https://github.com/fastify/fast-json-stringify/issues/246
                // anyOf: [
                //   {
                //     type: 'string',
                //   },
                //   // {
                //   //   type: 'object',
                //   // },
                //   {
                //     type: 'array',
                //     //items: {},
                //     additionalProperties: true,
                //   },
                //   {
                //     type: 'integer',
                //   },
                //   {
                //     type: 'number',
                //   },
                //   {
                //     type: 'boolean',
                //   },
                // ],
                //additionalProperties: true,
              },
            },
            required: ['tx_hash', 'tx_index', 'block_height', 'payload'],
          },
        };
      }

      // 3 bug -> edge case
      if (endpointName === '/nutlink/tickers/{ticker}') {
        responses.response[200] = {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              address: {
                type: 'string',
                description: 'Address of a metadata oracle',
              },
              tx_hash: {
                type: 'string',
                description: 'Hash of the transaction',
              },
              block_height: {
                type: 'integer',
                description: 'Block height of the record',
              },
              tx_index: {
                type: 'integer',
                description: 'Transaction index within the block',
              },
              payload: {
                // possible bug FIXME https://github.com/fastify/fast-json-stringify/issues/246
                // anyOf: [
                //   {
                //     type: 'string',
                //   },
                //   // {
                //   //   type: 'object',
                //   // },
                //   {
                //     type: 'array',
                //     //items: {},
                //     additionalProperties: true,
                //   },
                //   {
                //     type: 'integer',
                //   },
                //   {
                //     type: 'number',
                //   },
                //   {
                //     type: 'boolean',
                //   },
                // ],
                //additionalProperties: true,
              },
            },
            required: [
              'address',
              'tx_hash',
              'block_height',
              'tx_index',
              'payload',
            ],
          },
        };
      }

      // 4 bug -> edge case
      if (endpointName === '/scripts/{script_hash}/json') {
        responses.response[200] = {
          type: 'object',
          properties: {
            json_value: {
              nullable: true,
            },
          },
          required: ['json'],
        };
      }
    }

    // errors and others
    else {
      responses.response[response] =
        spec.components.responses[response].content['application/json'].schema;
    }
  }

  if (endpointName === '/addresses/{address}') {
    console.log(JSON.stringify(responses));
  }
  return responses;
};

export type { paths, components } from './generated-types';
