import { tbdexError } from "../../errors";
import { Rfq } from "../../messages/rfq";
import wasm from "../../wasm";

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
