import { tbdexError } from "../../errors";
import { Close } from "../../messages/close";
import { OrderInstructions } from "../../messages/order-instructions";
import { OrderStatus } from "../../messages/order-status";
import { Quote } from "../../messages/quote";
import wasm from "../../wasm";

export type ReplyToMessage = Quote | OrderInstructions | OrderStatus | Close;

export class ReplyToRequestBody {
  readonly message: ReplyToMessage;

  constructor(message: ReplyToMessage) {
    this.message = message;
  }

  static fromWASM = (
    wasmReplyToRequestBody: wasm.WasmReplyToRequestBody
  ): ReplyToRequestBody => {
    try {
      const kind = wasmReplyToRequestBody.data.kind;
      const json = wasmReplyToRequestBody.data.json;

      let message: ReplyToMessage;

      if (kind === "quote") message = Quote.fromJSONString(json);
      else if (kind === "orderinstructions")
        message = OrderInstructions.fromJSONString(json);
      else if (kind === "orderstatus")
        message = OrderStatus.fromJSONString(json);
      else if (kind === "close") message = Close.fromJSONString(json);
      else throw Error(`unknown kind ${kind}`);

      return new ReplyToRequestBody(message);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmReplyToRequestBody => {
    try {
      return new wasm.WasmReplyToRequestBody(this.message);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): ReplyToRequestBody => {
    try {
      return ReplyToRequestBody.fromWASM(
        wasm.WasmReplyToRequestBody.from_json_string(json)
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
