import { validateSchema } from '../index';
import { CIPTypes, GetOnchainMetadataResult, Asset } from '../types/metadata';

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

export const getOnchainMetadata = (
  onchainMetadata: Asset['onchain_metadata'],
  assetName: Asset['asset_name'],
  policyId: Asset['policy_id'],
): GetOnchainMetadataResult => {
  let internalOnchainMetada: any = onchainMetadata;

  if (!internalOnchainMetada)
    return { onchainMetadata: null, validCIPversion: null };

  let isFound = false;
  let onchainMetadataResult = null;
  let validCIPversion: CIPTypes = null;
  const version = getOnchainMetadataVersion(onchainMetadata);
  const assetNameBase = assetName || '';
  const assetNameVersion1 = Buffer.from(assetNameBase || '', 'hex').toString(
    'utf8',
  );
  const assetNameVersion2 = assetNameBase;

  if (version === 1) {
    try {
      onchainMetadataResult =
        internalOnchainMetada[policyId][assetNameVersion1] || null;
      isFound = true;
    } catch (error) {
      onchainMetadataResult = null;
    }
  }

  if (version === 2) {
    try {
      const foundMetadata =
        internalOnchainMetada[policyId][assetNameVersion2] || null;
      if (foundMetadata) {
        onchainMetadataResult = foundMetadata;
        isFound = true;
      } else {
        // fallback
        onchainMetadataResult =
          internalOnchainMetada[policyId][assetNameVersion1] || null;
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
