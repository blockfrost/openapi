export default {
	type: "array",
	items: {
		type: "object",
		properties: {
			label: {
				type: "string",
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
		required: ["label", "json_metadata"],
	},
};
