export default {
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
    required: ['address', 'tx_hash', 'block_height', 'tx_index', 'payload'],
  },
};
