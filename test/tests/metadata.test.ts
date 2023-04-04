import { expect, describe, test } from 'vitest';
import * as lib from '../../src/index';
import * as fixtures from '../fixtures/metadata';

describe('metadata functions', () => {
  fixtures.getOnchainMetadata.map(fixture => {
    test(fixture.name, async () => {
      const result = lib.getOnchainMetadata(
        fixture.data.onchain_metadata,
        fixture.data.asset_name,
        fixture.data.policy_id,
        fixture.data.onchain_metadata_cbor ?? null,
      );

      expect(result).toStrictEqual(fixture.response);
    });
  });

  expect(lib.getCIPstandard(1, false)).toStrictEqual(null);
  expect(lib.getCIPstandard(2, false)).toStrictEqual(null);
  expect(lib.getCIPstandard(1, true)).toStrictEqual('CIP25v1');
  expect(lib.getCIPstandard(2, true)).toStrictEqual('CIP25v2');
  expect(lib.getCIPstandard(3, false)).toStrictEqual(null);

  fixtures.validateCIP68Metadata.map(fixture => {
    test(fixture.name, () => {
      const result = lib.validateCIP68Metadata(
        // @ts-expect-error non number version
        fixture.payload,
        fixture.standard,
      );

      expect(result).toStrictEqual(fixture.response);
    });
  });
});
