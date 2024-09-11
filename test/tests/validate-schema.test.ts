import { describe, expect, test } from "vitest";
import { validateSchema } from "../../src/index";

describe("validateSchema", () => {
	test("valid blocks/latest data", () => {
		expect(
			validateSchema("block_content", {
				time: 1641338934,
				height: 15243593,
				hash: "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
				slot: 412162133,
				epoch: 425,
				epoch_slot: 12,
				slot_leader: "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
				size: 3,
				tx_count: 1,
				output: "128314491794",
				fees: "592661",
				block_vrf:
					"vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
				op_cert:
					"da905277534faf75dae41732650568af545134ee08a3c0392dbefc8096ae177c",
				op_cert_counter: "18",
				previous_block:
					"43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
				next_block:
					"8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
				confirmations: 4698,
			}),
		).toStrictEqual({ errors: null, isValid: true });
	});

	test("NOT valid blocks/latest data (timse shoudl be time)", () => {
		expect(
			validateSchema("block_content", {
				times: 1641338934,
				height: 15243593,
				hash: "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
				slot: 412162133,
				epoch: 425,
				epoch_slot: 12,
				slot_leader: "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
				size: 3,
				tx_count: 1,
				output: "128314491794",
				fees: "592661",
				block_vrf:
					"vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
				op_cert:
					"da905277534faf75dae41732650568af545134ee08a3c0392dbefc8096ae177c",
				op_cert_counter: "18",
				previous_block:
					"43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
				next_block:
					"8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
				confirmations: 4698,
			}),
		).toStrictEqual({
			isValid: false,
			errors: [
				{
					instancePath: "",
					keyword: "required",
					message: "must have required property 'time'",
					params: {
						missingProperty: "time",
					},
					schemaPath: "#/required",
				},
			],
		});
	});
});
