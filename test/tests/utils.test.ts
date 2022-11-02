import { expect, describe, test } from 'vitest';
import { getSchemaForEndpoint } from '../../src/index';
import {
  error400,
  error403,
  error418,
  error429,
  error500,
  error404,
} from '../fixtures/openapi';

describe('open api utils', () => {
  test('health schema - no refs', () => {
    expect(getSchemaForEndpoint('/health')).toStrictEqual({
      response: {
        '200': {
          properties: {
            is_healthy: {
              example: true,
              type: 'boolean',
            },
          },
          required: ['is_healthy'],
          type: 'object',
        },
        '400': error400,
        '403': error403,
        '418': error418,
        '429': error429,
        '500': error500,
      },
    });
  });

  test('epochs - nested refs', () => {
    expect(getSchemaForEndpoint('/epochs/{number}/previous')).toStrictEqual({
      params: {
        type: 'object',
        properties: {
          number: {
            type: 'integer',
          },
        },
      },
      querystring: {
        type: 'object',
        properties: {
          count: {
            default: 100,
            maximum: 100,
            minimum: 1,
            type: 'integer',
          },
          page: {
            default: 1,
            maximum: 21474836,
            minimum: 1,
            type: 'integer',
          },
        },
      },
      response: {
        '200': {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              epoch: {
                type: 'integer',
                description: 'Epoch number',
                example: 225,
              },
              start_time: {
                type: 'integer',
                description: 'Unix time of the start of the epoch',
                example: 1603403091,
              },
              end_time: {
                type: 'integer',
                description: 'Unix time of the end of the epoch',
                example: 1603835086,
              },
              first_block_time: {
                type: 'integer',
                description: 'Unix time of the first block of the epoch',
                example: 1603403092,
              },
              last_block_time: {
                type: 'integer',
                description: 'Unix time of the last block of the epoch',
                example: 1603835084,
              },
              block_count: {
                type: 'integer',
                description: 'Number of blocks within the epoch',
                example: 21298,
              },
              tx_count: {
                type: 'integer',
                description: 'Number of transactions within the epoch',
                example: 17856,
              },
              output: {
                type: 'string',
                description:
                  'Sum of all the transactions within the epoch in Lovelaces',
                example: '7849943934049314',
              },
              fees: {
                type: 'string',
                description:
                  'Sum of all the fees within the epoch in Lovelaces',
                example: '4203312194',
              },
              active_stake: {
                nullable: true,
                type: 'string',
                description:
                  'Sum of all the active stakes within the epoch in Lovelaces',
                example: '784953934049314',
              },
            },
            required: [
              'epoch',
              'start_time',
              'end_time',
              'first_block_time',
              'last_block_time',
              'block_count',
              'tx_count',
              'output',
              'fees',
              'active_stake',
            ],
          },
        },
        '400': error400,
        '403': error403,
        '404': error404,
        '418': error418,
        '429': error429,
        '500': error500,
      },
    });
  });

  test('addresses/{address}', () => {
    expect(getSchemaForEndpoint('/addresses/{address}')).toStrictEqual({
      params: {
        type: 'object',
        properties: {
          address: { type: 'string' },
        },
      },
      response: {
        200: {
          type: 'object',
          properties: {
            address: {
              type: 'string',
              description: 'Bech32 encoded addresses',
              example:
                'addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz',
            },
            amount: {
              type: 'array',
              items: {
                type: 'object',
                description: 'The sum of all the UTXO per asset',
                properties: {
                  unit: {
                    type: 'string',
                    format:
                      'Lovelace or concatenation of asset policy_id and hex-encoded asset_name',
                    description: 'The unit of the value',
                  },
                  quantity: {
                    type: 'string',
                    description: 'The quantity of the unit',
                  },
                },
                required: ['unit', 'quantity'],
              },
              example: [
                { unit: 'lovelace', quantity: '42000000' },
                {
                  unit: 'b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e',
                  quantity: '12',
                },
              ],
            },
            stake_address: {
              type: 'string',
              nullable: true,
              example:
                'stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7',
              description: 'Stake address that controls the key',
            },
            type: {
              type: 'string',
              enum: ['byron', 'shelley'],
              example: 'shelley',
              description: 'Address era',
            },
            script: {
              type: 'boolean',
              example: false,
              description: 'True if this is a script address',
            },
          },
          required: ['address', 'amount', 'stake_address', 'type', 'script'],
        },
        400: error400,
        404: error404,
        418: error418,
        403: error403,
        500: error500,
        429: error429,
      },
    });
  });

  test('anyOf case - all refs', () => {
    expect(getSchemaForEndpoint('/pools/{pool_id}/metadata')).toStrictEqual({
      params: {
        properties: {
          pool_id: {
            type: 'string',
          },
        },
        type: 'object',
      },
      response: {
        '200': {
          anyOf: [
            {
              properties: {
                description: {
                  description: 'Description of the stake pool',
                  example: 'The best pool ever',
                  nullable: true,
                  type: 'string',
                },
                hash: {
                  description: 'Hash of the metadata file',
                  example:
                    '47c0c68cb57f4a5b4a87bad896fc274678e7aea98e200fa14a1cb40c0cab1d8c',
                  nullable: true,
                  type: 'string',
                },
                hex: {
                  description: 'Hexadecimal pool ID',
                  example:
                    '0f292fcaa02b8b2f9b3c8f9fd8e0bb21abedb692a6d5058df3ef2735',
                  type: 'string',
                },
                homepage: {
                  description: 'Home page of the stake pool',
                  example: 'https://stakentus.com/',
                  nullable: true,
                  type: 'string',
                },
                name: {
                  description: 'Name of the stake pool',
                  example: 'Stake Nuts',
                  nullable: true,
                  type: 'string',
                },
                pool_id: {
                  description: 'Bech32 pool ID',
                  example:
                    'pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy',
                  type: 'string',
                },
                ticker: {
                  description: 'Ticker of the stake pool',
                  example: 'NUTS',
                  nullable: true,
                  type: 'string',
                },
                url: {
                  description: 'URL to the stake pool metadata',
                  example: 'https://stakenuts.com/mainnet.json',
                  nullable: true,
                  type: 'string',
                },
              },
              required: [
                'pool_id',
                'hex',
                'url',
                'hash',
                'ticker',
                'name',
                'description',
                'homepage',
              ],
              type: 'object',
            },
            { type: 'object' },
          ],
        },
        '400': error400,
        '403': error403,
        '418': error418,
        '429': error429,
        '404': error404,
        '500': error500,
      },
    });
  });

  test('array and refs', () => {
    expect(getSchemaForEndpoint('/epochs/{number}/next')).toStrictEqual({
      params: {
        properties: {
          number: {
            type: 'integer',
          },
        },
        type: 'object',
      },
      querystring: {
        type: 'object',
        properties: {
          count: {
            default: 100,
            maximum: 100,
            minimum: 1,
            type: 'integer',
          },
          page: {
            default: 1,
            maximum: 21474836,
            minimum: 1,
            type: 'integer',
          },
        },
      },
      response: {
        '200': {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              epoch: {
                type: 'integer',
                description: 'Epoch number',
                example: 225,
              },
              start_time: {
                type: 'integer',
                description: 'Unix time of the start of the epoch',
                example: 1603403091,
              },
              end_time: {
                description: 'Unix time of the end of the epoch',
                example: 1603835086,
                type: 'integer',
              },
              first_block_time: {
                type: 'integer',
                description: 'Unix time of the first block of the epoch',
                example: 1603403092,
              },
              last_block_time: {
                type: 'integer',
                description: 'Unix time of the last block of the epoch',
                example: 1603835084,
              },
              block_count: {
                type: 'integer',
                description: 'Number of blocks within the epoch',
                example: 21298,
              },
              tx_count: {
                type: 'integer',
                description: 'Number of transactions within the epoch',
                example: 17856,
              },
              output: {
                type: 'string',
                description:
                  'Sum of all the transactions within the epoch in Lovelaces',
                example: '7849943934049314',
              },
              fees: {
                type: 'string',
                description:
                  'Sum of all the fees within the epoch in Lovelaces',
                example: '4203312194',
              },
              active_stake: {
                description:
                  'Sum of all the active stakes within the epoch in Lovelaces',
                example: '784953934049314',
                nullable: true,
                type: 'string',
              },
            },
            required: [
              'epoch',
              'start_time',
              'end_time',
              'first_block_time',
              'last_block_time',
              'block_count',
              'tx_count',
              'output',
              'fees',
              'active_stake',
            ],
          },
        },
        '400': error400,
        '403': error403,
        '418': error418,
        '404': error404,
        '429': error429,
        '500': error500,
      },
    });
  });
});
