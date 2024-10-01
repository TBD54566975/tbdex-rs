import { tbdexError } from "../errors";
import { Offering } from "../resources/offering";
import wasm from "../wasm";

export class GetOfferingsResponseBody {
  readonly data: Offering[];

  constructor(data: Offering[]) {
    this.data = data;
  }

  static fromWASM = (
    wasmGetOfferingsResponseBody: wasm.WasmGetOfferingsResponseBody
  ): GetOfferingsResponseBody => {
    try {
      return new GetOfferingsResponseBody(
        wasmGetOfferingsResponseBody.data.map(Offering.fromWASM)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmGetOfferingsResponseBody => {
    try {
      return new wasm.WasmGetOfferingsResponseBody(
        this.data.map((o) => o.toWASM())
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): GetOfferingsResponseBody => {
    try {
      return GetOfferingsResponseBody.fromWASM(
        wasm.WasmGetOfferingsResponseBody.from_json_string(json)
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
