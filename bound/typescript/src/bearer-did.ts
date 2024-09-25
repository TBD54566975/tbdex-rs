import { withError } from "./errors";
import { KeyManager } from "./key-managers";
import { Signer } from "./signers";
import { WasmBearerDid } from "./wasm/generated";
import { Did, Document, PortableDid } from "./wasm/mappings";

export class BearerDid {
  readonly did: Did;
  readonly document: Document;
  readonly keyManager: KeyManager;

  constructor(did: Did, document: Document, keyManager: KeyManager) {
    this.did = did;
    this.document = document;
    this.keyManager = keyManager;
  }

  private static fromWASM = withError(
    (wasmBearerDid: WasmBearerDid): BearerDid => {
      return new BearerDid(
        Did.fromWASM(wasmBearerDid.did),
        Document.fromWASM(wasmBearerDid.document),
        KeyManager.fromWASM(wasmBearerDid.key_manager)
      );
    }
  );

  private toWASM = withError((): WasmBearerDid => {
    return new WasmBearerDid(
      Did.toWASM(this.did),
      Document.toWASM(this.document),
      KeyManager.toWASM(this.keyManager)
    );
  });

  static fromPortableDID = withError((portableDID: PortableDid): BearerDid => {
    return BearerDid.fromWASM(
      WasmBearerDid.from_portable_did(PortableDid.toWASM(portableDID))
    );
  });

  getSigner = withError((verificationMethodId: string): Signer => {
    return Signer.fromWASM(this.toWASM().get_signer(verificationMethodId));
  });
}
