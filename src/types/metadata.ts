import { components } from '../generated-types';

type Schemas = components['schemas'];
type Asset = Schemas['asset'];

export type GetOnchainMetadataResult = {
  onchainMetadata: Schemas['onchain_metadata_cip25'] | null;
  validCIPversion: CIPTypes;
};

export type CIPTypes = 'CIP25v1' | 'CIP25v2' | null;

export type { Schemas, Asset };
