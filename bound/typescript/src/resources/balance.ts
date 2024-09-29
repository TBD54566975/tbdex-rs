import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { BalanceData, ResourceMetadata } from "../wasm/mappings";

export class Balance {
  readonly metadata: ResourceMetadata;
  readonly data: BalanceData;
  signature: string;

  constructor(
    metadata: ResourceMetadata,
    data: BalanceData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromWASM = (wasmBalance: wasm.WasmBalance): Balance => {
    try {
      return new Balance(
        ResourceMetadata.fromWASM(wasmBalance.metadata),
        BalanceData.fromWASM(wasmBalance.data),
        wasmBalance.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmBalance => {
    try {
      return new wasm.WasmBalance(
        ResourceMetadata.toWASM(this.metadata),
        BalanceData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Balance => {
    try {
      return Balance.fromWASM(wasm.WasmBalance.from_json_string(json));
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
    from: string,
    data: BalanceData,
    protocol?: string
  ): Balance => {
    try {
      return Balance.fromWASM(
        wasm.WasmBalance.create(from, BalanceData.toWASM(data), protocol)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const wasmBalance = this.toWASM();
      wasmBalance.sign(bearerDid.toWASM());
      this.signature = wasmBalance.signature;
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
