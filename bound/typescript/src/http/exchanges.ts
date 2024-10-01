import { tbdexError } from "../errors";
import { Message } from "../messages";
import { Cancel } from "../messages/cancel";
import { Close } from "../messages/close";
import { Order } from "../messages/order";
import { OrderInstructions } from "../messages/order-instructions";
import { OrderStatus } from "../messages/order-status";
import { Quote } from "../messages/quote";
import { Rfq } from "../messages/rfq";
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

export class CreateExchangeRequestBody {
  readonly message: Rfq;
  readonly replyTo?: string;

  constructor(message: Rfq, replyTo?: string) {
    this.message = message;
    this.replyTo = replyTo;
  }

  static fromWASM = (
    wasmCreateExchangeRequestBody: wasm.WasmCreateExchangeRequestBody
  ): CreateExchangeRequestBody => {
    try {
      return new CreateExchangeRequestBody(
        Rfq.fromWASM(wasmCreateExchangeRequestBody.message),
        wasmCreateExchangeRequestBody.reply_to
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmCreateExchangeRequestBody => {
    try {
      return new wasm.WasmCreateExchangeRequestBody(
        this.message.toWASM(),
        this.replyTo
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): CreateExchangeRequestBody => {
    try {
      return CreateExchangeRequestBody.fromWASM(
        wasm.WasmCreateExchangeRequestBody.from_json_string(json)
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
