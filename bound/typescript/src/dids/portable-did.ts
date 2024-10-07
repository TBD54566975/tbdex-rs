import { Jwk } from "../crypto/jwk";
import { Document } from "./document";

export type PortableDid = {
  uri: string;
  document: Document;
  privateKeys: Jwk[];
};
