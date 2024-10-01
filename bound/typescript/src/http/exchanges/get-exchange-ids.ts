import { tbdexError } from "../../errors";
import wasm from "../../wasm";

export class GetExchangesResponseBody {
  readonly data: string[];

  constructor(data: string[]) {
    this.data = data;
  }

  static fromWASM = (
    wasmGetExchangesResponseBody: wasm.WasmGetExchangesResponseBody
  ): GetExchangesResponseBody => {
    try {
      return new GetExchangesResponseBody(wasmGetExchangesResponseBody.data);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmGetExchangesResponseBody => {
    try {
      return new wasm.WasmGetExchangesResponseBody(this.data);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): GetExchangesResponseBody => {
    try {
      return GetExchangesResponseBody.fromWASM(
        wasm.WasmGetExchangesResponseBody.from_json_string(json)
      );
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
}
