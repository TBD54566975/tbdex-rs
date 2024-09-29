import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, CancelData } from "../wasm/mappings";

export class Cancel {
  readonly metadata: MessageMetadata;
  readonly data: CancelData;
  signature: string;

  constructor(metadata: MessageMetadata, data: CancelData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromWASM = (wasmCancel: wasm.WasmCancel): Cancel => {
    try {
      return new Cancel(
        MessageMetadata.fromWASM(wasmCancel.metadata),
        CancelData.fromWASM(wasmCancel.data),
        wasmCancel.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmCancel => {
    try {
      return new wasm.WasmCancel(
        MessageMetadata.toWASM(this.metadata),
        CancelData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Cancel => {
    try {
      return Cancel.fromWASM(wasm.WasmCancel.from_json_string(json));
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
    data: CancelData,
    protocol?: string,
    externalId?: string
  ): Cancel => {
    try {
      return Cancel.fromWASM(
        wasm.WasmCancel.create(
          to,
          from,
          exchangeId,
          CancelData.toWASM(data),
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
      const wasmCancel = this.toWASM();
      wasmCancel.sign(bearerDid.toWASM());
      this.signature = wasmCancel.signature;
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
