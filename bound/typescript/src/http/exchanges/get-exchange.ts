import { tbdexError } from "../../errors";
import { Message } from "../../messages";
import { Cancel } from "../../messages/cancel";
import { Close } from "../../messages/close";
import { Order } from "../../messages/order";
import { OrderInstructions } from "../../messages/order-instructions";
import { OrderStatus } from "../../messages/order-status";
import { Quote } from "../../messages/quote";
import { Rfq } from "../../messages/rfq";
import wasm from "../../wasm";

export class GetExchangeResponseBody {
  readonly data: Message[];

  constructor(data: Message[]) {
    this.data = data;
  }

  static fromWASM = (
    wasmGetExchangeResponseBody: wasm.WasmGetExchangeResponseBody
  ): GetExchangeResponseBody => {
    try {
      return new GetExchangeResponseBody(
        wasmGetExchangeResponseBody.data.map(({ kind, json }) => {
          if (kind === "rfq") return Rfq.fromJSONString(json);
          else if (kind === "quote") return Quote.fromJSONString(json);
          else if (kind === "order") return Order.fromJSONString(json);
          else if (kind === "orderinstructions")
            return OrderInstructions.fromJSONString(json);
          else if (kind === "cancel") return Cancel.fromJSONString(json);
          else if (kind === "orderstatus")
            return OrderStatus.fromJSONString(json);
          else if (kind === "close") return Close.fromJSONString(json);

          throw Error(`unknown kind ${kind}`);
        })
      );
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
