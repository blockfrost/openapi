import { components } from '../generated-types';

export type Schemas = components['schemas'];
export type Asset = Schemas['asset'];

export type GetOnchainMetadataResult = {
  onchainMetadata: Schemas['onchain_metadata_cip25'] | null;
  validCIPversion: CIPTypes;
};

export type CIPTypes = 'CIP25v1' | 'CIP25v2' | null;
export type CIP68Version = 'CIP68v1' | null;

export type validateCIP68MetadataInput = {
  metadata: unknown;
  version: number;
} | null;

export type validateCIP68MetadataOverload = {
  (input: validateCIP68MetadataInput, type: 'ft'):
    | {
        version: 'CIP68v1';
        metadata: Schemas['onchain_metadata_cip68_ft_333'];
      }
    | false;
  (input: validateCIP68MetadataInput, type: 'nft'):
    | {
        version: 'CIP68v1';
        metadata: Schemas['onchain_metadata_cip68_nft_222'];
      }
    | false;
  (input: validateCIP68MetadataInput, type: 'nft' | 'ft'):
    | {
        version: 'CIP68v1';
        metadata:
          | Schemas['onchain_metadata_cip68_nft_222']
          | Schemas['onchain_metadata_cip68_ft_333'];
      }
    | false;
};
