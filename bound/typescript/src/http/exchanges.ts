import { tbdexError } from "../errors";
import { Message } from "../messages";
import wasm from "../wasm";

export class GetExchangeResponseBody {
  readonly data: Message[];

  constructor(data: Message[]) {
    this.data = data;
  }

  static fromWASM = (
    wasmGetExchangeResponseBody: wasm.WasmGetExchangeResponseBody
  ): GetExchangeResponseBody => {
    try {
      return new GetExchangeResponseBody(wasmGetExchangeResponseBody.data);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmGetExchangeResponseBody => {
    try {
      return new wasm.WasmGetExchangeResponseBody(this.data);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): GetExchangeResponseBody => {
    try {
      return GetExchangeResponseBody.fromWASM(
        wasm.WasmGetExchangeResponseBody.from_json_string(json)
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
