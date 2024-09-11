export default {
	type: "array",
	items: {
		type: "object",
		properties: {
			tx_hash: {
				type: "string",
			},
			block_height: {
				type: "integer",
			},
			tx_index: {
				type: "integer",
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
		required: ["tx_hash", "tx_index", "block_height", "payload"],
	},
};
