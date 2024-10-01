import { tbdexError } from "../errors";
import { Balance } from "../resources/balance";
import wasm from "../wasm";

export class GetBalancesResponseBody {
  readonly data: Balance[];

  constructor(data: Balance[]) {
    this.data = data;
  }

  static fromWASM = (
    wasmGetBalancesResponseBody: wasm.WasmGetBalancesResponseBody
  ): GetBalancesResponseBody => {
    try {
      return new GetBalancesResponseBody(
        wasmGetBalancesResponseBody.data.map(Balance.fromWASM)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmGetBalancesResponseBody => {
    try {
      return new wasm.WasmGetBalancesResponseBody(
        this.data.map((o) => o.toWASM())
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): GetBalancesResponseBody => {
    try {
      return GetBalancesResponseBody.fromWASM(
        wasm.WasmGetBalancesResponseBody.from_json_string(json)
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
