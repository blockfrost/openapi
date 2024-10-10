import { components } from '../generated-types';

export type Schemas = components['schemas'];
export type Asset = Schemas['asset'];

export type GetOnchainMetadataResult = {
  onchainMetadata: Schemas['onchain_metadata_cip25'] | null;
  validCIPversion: CIPTypes;
};

export type CIPTypes = 'CIP25v1' | 'CIP25v2' | null;
export type ValidCIP68Version = 'CIP68v1' | 'CIP68v2' | 'CIP68v3';
export type CIP68Version = ValidCIP68Version | null;

export type validateCIP68MetadataInput = {
  metadata: unknown;
  version: number;
  extra: string | undefined;
} | null;

export type validateCIP68MetadataOverload = {
  (input: validateCIP68MetadataInput, type: 'ft'):
    | {
        version: ValidCIP68Version;
        extra: string | undefined;
        metadata: Schemas['onchain_metadata_cip68_ft_333'];
      }
    | false;
  (input: validateCIP68MetadataInput, type: 'nft'):
    | {
        version: ValidCIP68Version;
        extra: string | undefined;
        metadata: Schemas['onchain_metadata_cip68_nft_222'];
      }
    | false;
  (input: validateCIP68MetadataInput, type: 'rft'):
    | {
        version: ValidCIP68Version;
        extra: string | undefined;
        metadata: Schemas['onchain_metadata_cip68_rft_444'];
      }
    | false;
  (input: validateCIP68MetadataInput, type: 'nft' | 'ft' | 'rft'):
    | {
        version: ValidCIP68Version;
        extra: string | undefined;
        metadata:
          | Schemas['onchain_metadata_cip68_nft_222']
          | Schemas['onchain_metadata_cip68_ft_333']
          | Schemas['onchain_metadata_cip68_rft_444'];
      }
    | false;
};
