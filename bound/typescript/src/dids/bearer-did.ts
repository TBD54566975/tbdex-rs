import { PortableDid } from "./portable-did";
import wasm from "../wasm";
import { Document } from "./document";
import { KeyManager } from "../crypto/key-manager";
import { tbdexError } from "../errors";
import { InMemoryKeyManager } from "../crypto/in-memory-key-manager";
import { Signer } from "../crypto/signer";

export class BearerDid {
  readonly uri: string;
  readonly document: Document;
  readonly keyManager: KeyManager;

  constructor(uri: string, document: Document, keyManager: KeyManager) {
    this.uri = uri;
    this.document = document;
    this.keyManager = keyManager;
  }

  toWASM = (): wasm.WasmBearerDid => {
    try {
      return new wasm.WasmBearerDid(
        this.uri,
        JSON.stringify(this.document),
        KeyManager.toWASM(this.keyManager)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromPortableDID = (portableDID: PortableDid): BearerDid => {
    try {
      const keyManager = new InMemoryKeyManager();
      portableDID.privateKeys.forEach((privateJwk) =>
        keyManager.importPrivateJwk(privateJwk)
      );
      return new BearerDid(portableDID.uri, portableDID.document, keyManager);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  getSigner = (verificationMethodId: string): Signer => {
    try {
      return Signer.fromWASM(this.toWASM().get_signer(verificationMethodId));
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
