import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, CloseData } from "../wasm/generated-mappings";

export class Close {
  readonly metadata: MessageMetadata;
  readonly data: CloseData;
  signature: string;

  constructor(metadata: MessageMetadata, data: CloseData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromWASM = (wasmClose: wasm.WasmClose): Close => {
    try {
      return new Close(
        MessageMetadata.fromWASM(wasmClose.metadata),
        CloseData.fromWASM(wasmClose.data),
        wasmClose.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmClose => {
    try {
      return new wasm.WasmClose(
        MessageMetadata.toWASM(this.metadata),
        CloseData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Close => {
    try {
      return Close.fromWASM(wasm.WasmClose.from_json_string(json));
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toJSONString = (): string => {
    try {
      return this.toWASM().to_json_string();
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static create = (
    to: string,
    from: string,
    exchangeId: string,
    data: CloseData,
    protocol?: string,
    externalId?: string
  ): Close => {
    try {
      return Close.fromWASM(
        wasm.WasmClose.create(
          to,
          from,
          exchangeId,
          CloseData.toWASM(data),
          protocol,
          externalId
        )
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const wasmClose = this.toWASM();
      wasmClose.sign(bearerDid.toWASM());
      this.signature = wasmClose.signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = () => {
    try {
      this.toWASM().verify();
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
