import cbor from 'cbor';
import { validateSchema } from '../index';
import {
  CIPTypes,
  GetOnchainMetadataResult,
  Asset,
  validateCIP68MetadataOverload,
} from '../types/metadata';

export const getCIPstandard = (version: number, isValid: boolean): CIPTypes => {
  if (isValid) {
    if (version === 1) {
      return 'CIP25v1';
    }

    if (version === 2) {
      return 'CIP25v2';
    }
  }

  return null;
};

export const getOnchainMetadataVersion = (
  onchainMetadata: Asset['onchain_metadata'],
): number => {
  if (!onchainMetadata?.version) {
    return 1;
  }

  return Number(onchainMetadata.version);
};

export const findAssetInMetadataCBOR = (
  policyId: string,
  assetName: string,
  metadataCbor: string,
): unknown | undefined => {
  // Policy id and asset name for CIP25v2 assets should be encoded as bytes, which results in
  // JSON metadata from cardano-db-sync having "0x" prefix before the actual policy id and asset name.
  // We need to make sure that the actual metadata CBOR includes the correct asset policy id
  // and asset name, encoded as bytes. JSON metadata could also include "0x<policyId>" key if it was
  // encoded as CBOR string, instead of bytes. Metadata encoded in such way are not valid,
  // but there is no way to differentiate between "0x<policy_id>" encoded as string
  // and proper <policy_id> encoded as bytes using just JSON metadata.

  try {
    const onchain_metadata_cbor = metadataCbor
      ? cbor.decodeAllSync(metadataCbor)
      : null;

    if (!onchain_metadata_cbor) {
      return undefined;
    }
    // there might be multiple cbor objects passed into decodeAllSync, that's why it returns an array
    const cbor_metadata_policies = onchain_metadata_cbor[0].get(721);

    for (const [policyKey, value] of cbor_metadata_policies) {
      if (
        !Buffer.isBuffer(policyKey) ||
        policyKey.toString('hex') !== policyId
      ) {
        continue;
      }

      if (!(value instanceof Map)) {
        // value needs to be map of {assetName: metadataObject}
        continue;
      }

      // value is map with asset names as keys
      for (const [assetNameKey, assetMetadata] of value) {
        if (
          !Buffer.isBuffer(assetNameKey) ||
          assetNameKey.toString('hex') !== assetName
        ) {
          continue;
        }

        return assetMetadata;
      }
    }
  } catch (error) {
    console.error(
      `Error while parsing metadata CBOR for ${policyId}${assetName}`,
      error,
    );
  }

  return undefined;
};

export const getOnchainMetadata = (
  onchainMetadata: Asset['onchain_metadata'],
  assetName: Asset['asset_name'],
  policyId: Asset['policy_id'],
  onchainMetadataCbor: string | null,
): GetOnchainMetadataResult => {
  let internalOnchainMetadata: any = onchainMetadata;

  if (!internalOnchainMetadata || !onchainMetadataCbor)
    return { onchainMetadata: null, validCIPversion: null };

  let isFound = false;
  let onchainMetadataResult = null;
  let validCIPversion: CIPTypes = null;
  const version = getOnchainMetadataVersion(onchainMetadata);
  const assetNameBase = assetName || '';
  const assetNameVersion1 = Buffer.from(assetNameBase || '', 'hex').toString(
    'utf8',
  );

  if (version === 1) {
    try {
      onchainMetadataResult =
        internalOnchainMetadata[policyId][assetNameVersion1] || null;
      isFound = true;
    } catch (error) {
      onchainMetadataResult = null;
    }
  }

  if (version === 2) {
    try {
      const assetNameVersion2 = `0x${assetNameBase}`;
      const policyIdVersion2 = `0x${policyId}`;

      const foundAssetInCbor = findAssetInMetadataCBOR(
        policyId,
        assetNameBase,
        onchainMetadataCbor,
      );
      const foundMetadata = foundAssetInCbor
        ? internalOnchainMetadata[policyIdVersion2][assetNameVersion2]
        : null;
      if (foundMetadata) {
        onchainMetadataResult = foundMetadata;
        isFound = true;
      } else {
        // fallback
        onchainMetadataResult =
          internalOnchainMetadata[policyId][assetNameVersion1] || null;
        isFound = false;
      }
    } catch (error) {
      onchainMetadataResult = null;
    }
  }

  const { isValid } = validateSchema(
    'asset_onchain_metadata_cip25',
    onchainMetadataResult,
  );

  const CIPVersion = getCIPstandard(version, isFound && isValid);

  validCIPversion = CIPVersion;

  return {
    onchainMetadata: onchainMetadataResult,
    validCIPversion,
  };
};

export const validateCIP68Metadata: validateCIP68MetadataOverload = (
  input: any,
  schema: any,
) => {
  if (!input) return false;
  if (input.version !== 1) return false;

  if (schema === 'nft') {
    const { isValid: isValidNFT } = validateSchema(
      'asset_onchain_metadata_cip68_nft_222',
      input.metadata,
    );

    return isValidNFT
      ? {
          version: 'CIP68v1',
          metadata: input.metadata,
        }
      : false;
  } else if (schema === 'ft') {
    const { isValid: isValidFT } = validateSchema(
      'asset_onchain_metadata_cip68_ft_333',
      input.metadata,
    );

    return isValidFT
      ? {
          version: 'CIP68v1',
          metadata: input.metadata,
        }
      : false;
  } else {
    return false;
  }
};
