export const transformSchemaElement = [
	{
		description: "perfectly fine object that does not need transformation",
		data: {
			type: "object",
			properties: {
				key: {
					type: "object",
					properties: {
						nestedkey: "string",
					},
				},
			},
		},
		result: {
			properties: {
				key: {
					properties: {
						nestedkey: "string",
					},
					type: "object",
				},
			},
			type: "object",
		},
	},
	{
		description: "nullable object",
		data: {
			type: "object",
			properties: {
				key: {
					type: ["object", "null"],
					properties: {
						nestedkey: "string",
					},
				},
			},
		},
		result: {
			properties: {
				key: {
					nullable: true,
					properties: {
						nestedkey: "string",
					},
					type: "object",
				},
			},
			type: "object",
		},
	},
	{
		description: "array with nullable object",
		data: {
			type: "array",
			items: {
				type: "object",
				properties: {
					key: {
						type: ["object", "null"],
						properties: {
							nestedkey: "string",
						},
					},
				},
			},
		},
		result: {
			type: "array",
			items: {
				properties: {
					key: {
						nullable: true,
						properties: {
							nestedkey: "string",
						},
						type: "object",
					},
				},
				type: "object",
			},
		},
	},
	{
		description: "object with array type of length 1",
		data: {
			type: "object",
			properties: {
				key: {
					type: ["object"],
					properties: {
						nestedkey: "string",
					},
				},
			},
		},
		result: {
			properties: {
				key: {
					properties: {
						nestedkey: "string",
					},
					type: "object",
				},
			},
			type: "object",
		},
	},
	{
		description: "object with nested arbitrary object",
		data: {
			type: "object",
			properties: {
				key: {
					type: "object",
					additionalProperties: true,
				},
			},
		},
		result: {
			type: "object",
			properties: {
				key: {},
			},
		},
	},
];

export const transformSchemaElementError = [
	{
		description: "array type with 2 types should throw",
		data: {
			type: ["object", "string"],
			properties: {
				key: {
					type: "object",
					properties: {
						nestedkey: "string",
					},
				},
			},
		},
		result: `Error in {"type":["object","string"],"properties":{"key":{"type":"object","properties":{"nestedkey":"string"}}}}. Type doesn't support an array with multiple values. Use anyOf/oneOf.`,
	},
	{
		description: "array type with multiple types should throw",
		data: {
			type: ["object", "string", "integer"],
			properties: {
				key: {
					type: "object",
					properties: {
						nestedkey: "string",
					},
				},
			},
		},
		result: `Error in {"type":["object","string","integer"],"properties":{"key":{"type":"object","properties":{"nestedkey":"string"}}}}. Type doesn't support an array with multiple values. Use anyOf/oneOf.`,
	},
];
