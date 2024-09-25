import { withError } from "./errors";
import { WasmBearerDid } from "./wasm/generated";
import { Document, PortableDid } from "./wasm/mappings";

export class BearerDid {
  readonly document: Document;
  // todo did and key_manager

  constructor(document: Document) {
    this.document = document;
  }

  private static fromWASM = withError(
    (wasmBearerDid: WasmBearerDid): BearerDid => {
      return new BearerDid(Document.fromWASM(wasmBearerDid.document));
    }
  );

  // todo need WasmBearerDid constructor
  // private toWASM = withError((): WasmBearerDid => {
  //   return new WasmBearerDid()
  // })

  static fromPortableDID = withError((portableDID: PortableDid): BearerDid => {
    return BearerDid.fromWASM(
      WasmBearerDid.from_portable_did(PortableDid.toWASM(portableDID))
    );
  });
}
