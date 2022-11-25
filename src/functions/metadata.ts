import { components } from '../generated-types';
import { validateSchema } from '../index';

export const getCIPstandard = (
  version: number,
  isValid: boolean,
): 'CIP25v1' | 'CIP25v2' | null => {
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
  onchainMetadata: components['schemas']['asset']['onchain_metadata'],
): number => {
  if (!onchainMetadata?.version) {
    return 1;
  }

  return Number(onchainMetadata.version);
};

export const getOnchainMetadata = (
  onchainMetadata: components['schemas']['asset']['onchain_metadata'],
  assetName: components['schemas']['asset']['asset_name'],
  policyId: components['schemas']['asset']['policy_id'],
) => {
  let internanOnchainMetada: any = onchainMetadata;
  if (!internanOnchainMetada) return null;

  let isFound = false;
  let onchainMetadataResult = null;
  let validCIPversion: 'CIP25v1' | 'CIP25v2' | null = null;
  const version = getOnchainMetadataVersion(onchainMetadata);
  const assetNameBase = assetName || '';
  const assetNameVersion1 = Buffer.from(assetNameBase || '', 'hex').toString(
    'utf8',
  );
  const assetNameVersion2 = assetNameBase;

  if (version === 1) {
    try {
      onchainMetadataResult =
        internanOnchainMetada[policyId][assetNameVersion1] || null;
      isFound = true;
    } catch (error) {
      onchainMetadataResult = null;
    }
  }

  if (version === 2) {
    try {
      const foundMetadata =
        internanOnchainMetada[policyId][assetNameVersion2] || null;
      if (foundMetadata) {
        onchainMetadataResult = foundMetadata;
        isFound = true;
      } else {
        // fallback
        onchainMetadataResult =
          internanOnchainMetada[policyId][assetNameVersion1] || null;
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
