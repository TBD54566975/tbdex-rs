import { withError } from "./errors";
import { WasmBearerDid } from "./wasm/generated";
import { Did, Document, PortableDid } from "./wasm/mappings";

export class BearerDid {
  readonly did: Did;
  readonly document: Document;
  // todo did and key_manager

  constructor(did: Did, document: Document) {
    this.did = did;
    this.document = document;
  }

  private static fromWASM = withError(
    (wasmBearerDid: WasmBearerDid): BearerDid => {
      return new BearerDid(
        Did.fromWASM(wasmBearerDid.did),
        Document.fromWASM(wasmBearerDid.document)
      );
    }
  );

  private toWASM = withError((): WasmBearerDid => {
    return new WasmBearerDid(
      Did.toWASM(this.did),
      Document.toWASM(this.document)
    );
  });

  static fromPortableDID = withError((portableDID: PortableDid): BearerDid => {
    return BearerDid.fromWASM(
      WasmBearerDid.from_portable_did(PortableDid.toWASM(portableDID))
    );
  });
}
