import { tbdexError } from "./errors";
import { KeyManager } from "./key-managers";
import { PortableDid } from "./portable-did";
import { Signer } from "./signers";
import wasm from "./wasm";
import { Did, Document } from "./wasm/mappings";

export class BearerDid {
  readonly did: Did;
  readonly document: Document;
  readonly keyManager: KeyManager;

  constructor(did: Did, document: Document, keyManager: KeyManager) {
    this.did = did;
    this.document = document;
    this.keyManager = keyManager;
  }

  static fromWASM = (wasmBearerDid: wasm.WasmBearerDid): BearerDid => {
    try {
      return new BearerDid(
        Did.fromWASM(wasmBearerDid.did),
        Document.fromWASM(wasmBearerDid.document),
        KeyManager.fromWASM(wasmBearerDid.key_manager)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmBearerDid => {
    try {
      return new wasm.WasmBearerDid(
        Did.toWASM(this.did),
        Document.toWASM(this.document),
        KeyManager.toWASM(this.keyManager)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromPortableDID = (portableDID: PortableDid): BearerDid => {
    try {
      return BearerDid.fromWASM(
        wasm.WasmBearerDid.from_portable_did(portableDID.toWASM())
      );
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
