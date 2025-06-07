import { EnvironmentsEnum } from 'lib';

export * from './sharedConfig';

export const API_URL = 'https://devnet-template-api.multiversx.com';
export const contractAddress =
  'erd1qqqqqqqqqqqqqpgqdkqukzw2f6gr9z74jzah50fqu5q53t4kdlls2ldqut';
export const environment = EnvironmentsEnum.devnet;
export const sampleAuthenticatedDomains = [API_URL];
