import wasm from "./wasm";
import { Document, Jwk } from "./wasm/mappings";

export class PortableDid {
  readonly uri: string;
  readonly document: Document;
  readonly privateKeys: Jwk[];

  constructor(uri: string, document: Document, privateKeys: Jwk[]) {
    this.uri = uri;
    this.document = document;
    this.privateKeys = privateKeys;
  }

  static fromWASM = (wasmPortableDid: wasm.WasmPortableDid): PortableDid => {
    return new PortableDid(
      wasmPortableDid.did_uri,
      Document.fromWASM(wasmPortableDid.document),
      wasmPortableDid.private_keys.map((x) => Jwk.fromWASM(x))
    );
  };

  toWASM = (): wasm.WasmPortableDid => {
    return new wasm.WasmPortableDid(
      this.uri,
      Document.toWASM(this.document),
      this.privateKeys.map((x) => Jwk.toWASM(x))
    );
  };

  static fromJSONString = (json: string): PortableDid => {
    return PortableDid.fromWASM(wasm.WasmPortableDid.from_json_string(json));
  };

  toJSONString = (): string => {
    return this.toWASM().to_json_string();
  };
}
