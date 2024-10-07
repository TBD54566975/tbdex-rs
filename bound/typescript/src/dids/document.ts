import { Jwk } from "../crypto/jwk";

export type Document = {
  alsoKnownAs?: string[];
  assertionMethod?: string[];
  authentication?: string[];
  capabilityDelegation?: string[];
  capabilityInvocation?: string[];
  context?: string[];
  controller?: string[];
  id: string;
  keyAgreement?: string[];
  service?: Service[];
  verificationMethod: VerificationMethod[];
};

export type Service = {
  id: string;
  serviceEndpoint: string[];
  type: string;
};

export type VerificationMethod = {
  controller: string;
  id: string;
  publicKeyJwk: Jwk;
  type: string;
};
