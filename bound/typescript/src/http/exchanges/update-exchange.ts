import { tbdexError } from "../../errors";
import { Cancel } from "../../messages/cancel";
import { Order } from "../../messages/order";
import wasm from "../../wasm";

export type WalletUpdateMessage = Order | Cancel;

export class UpdateExchangeRequestBody {
  readonly message: WalletUpdateMessage;

  constructor(message: WalletUpdateMessage) {
    this.message = message;
  }

  static fromWASM = (
    wasmUpdateExchangeRequestBody: wasm.WasmUpdateExchangeRequestBody
  ): UpdateExchangeRequestBody => {
    try {
      const kind = wasmUpdateExchangeRequestBody.data.kind;
      const json = wasmUpdateExchangeRequestBody.data.json;

      let message: WalletUpdateMessage;

      if (kind === "order") message = Order.fromJSONString(json);
      else if (kind === "cancel") message = Cancel.fromJSONString(json);
      else throw Error(`unknown kind ${kind}`);

      return new UpdateExchangeRequestBody(message);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmUpdateExchangeRequestBody => {
    try {
      return new wasm.WasmUpdateExchangeRequestBody(this.message);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): UpdateExchangeRequestBody => {
    try {
      return UpdateExchangeRequestBody.fromWASM(
        wasm.WasmUpdateExchangeRequestBody.from_json_string(json)
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
