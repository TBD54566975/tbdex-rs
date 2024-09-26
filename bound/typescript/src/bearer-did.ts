import { withError } from "./errors";
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

  static fromWASM = withError(
    (wasmBearerDid: wasm.WasmBearerDid): BearerDid => {
      return new BearerDid(
        Did.fromWASM(wasmBearerDid.did),
        Document.fromWASM(wasmBearerDid.document),
        KeyManager.fromWASM(wasmBearerDid.key_manager)
      );
    }
  );

  toWASM = withError((): wasm.WasmBearerDid => {
    return new wasm.WasmBearerDid(
      Did.toWASM(this.did),
      Document.toWASM(this.document),
      KeyManager.toWASM(this.keyManager)
    );
  });

  static fromPortableDID = withError((portableDID: PortableDid): BearerDid => {
    return BearerDid.fromWASM(
      wasm.WasmBearerDid.from_portable_did(portableDID.toWASM())
    );
  });

  getSigner = withError((verificationMethodId: string): Signer => {
    return Signer.fromWASM(this.toWASM().get_signer(verificationMethodId));
  });
}
