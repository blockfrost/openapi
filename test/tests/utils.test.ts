import { expect, describe, test } from 'vitest';
import { getSchemaForEndpoint } from '../../src/index';
import {
  error400,
  error403,
  error418,
  error429,
  error500,
} from '../fixtures/openapi';

describe('open api utils', () => {
  test('health schema - no refs', () => {
    expect(getSchemaForEndpoint('/health')).toMatchObject({
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
    expect(getSchemaForEndpoint('/epochs/{number}/previous')).toMatchObject({
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
              },
              start_time: {
                type: 'integer',
              },
              end_time: {
                type: 'integer',
                description: 'Unix time of the end of the epoch',
                example: 1603835086,
              },
              first_block_time: {
                type: 'integer',
              },
              last_block_time: {
                type: 'integer',
              },
              block_count: {
                type: 'integer',
                description: 'Number of blocks within the epoch',
                example: 21298,
              },
              tx_count: {
                type: 'integer',
              },
              output: {
                type: 'string',
              },
              fees: {
                type: 'string',
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
        '418': error418,
        '429': error429,
        '500': error500,
      },
    });
  });

  test('anyOf case - all refs', () => {
    expect(getSchemaForEndpoint('/pools/{pool_id}/metadata')).toMatchObject({
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
        '500': error500,
      },
    });
  });

  test('array and refs', () => {
    expect(getSchemaForEndpoint('/epochs/{number}/next')).toMatchObject({
      response: {
        '200': {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              epoch: {
                type: 'integer',
              },
              start_time: {
                type: 'integer',
              },
              end_time: {
                type: 'integer',
              },
              first_block_time: {
                type: 'integer',
              },
              last_block_time: {
                type: 'integer',
              },
              block_count: {
                type: 'integer',
              },
              tx_count: {
                type: 'integer',
              },
              output: {
                type: 'string',
              },
              fees: {
                type: 'string',
              },
              active_stake: {
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
        '429': error429,
        '500': error500,
      },
    });
  });
});
