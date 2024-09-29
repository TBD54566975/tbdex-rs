import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { MessageMetadata, QuoteData } from "../wasm/generated-mappings";

export class Quote {
  readonly metadata: MessageMetadata;
  readonly data: QuoteData;
  signature: string;

  constructor(metadata: MessageMetadata, data: QuoteData, signature: string) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromWASM = (wasmQuote: wasm.WasmQuote): Quote => {
    try {
      return new Quote(
        MessageMetadata.fromWASM(wasmQuote.metadata),
        QuoteData.fromWASM(wasmQuote.data),
        wasmQuote.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmQuote => {
    try {
      return new wasm.WasmQuote(
        MessageMetadata.toWASM(this.metadata),
        QuoteData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Quote => {
    try {
      return Quote.fromWASM(wasm.WasmQuote.from_json_string(json));
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
    to: string,
    from: string,
    exchangeId: string,
    data: QuoteData,
    protocol?: string,
    externalId?: string
  ): Quote => {
    try {
      return Quote.fromWASM(
        wasm.WasmQuote.create(
          to,
          from,
          exchangeId,
          QuoteData.toWASM(data),
          protocol,
          externalId
        )
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const wasmQuote = this.toWASM();
      wasmQuote.sign(bearerDid.toWASM());
      this.signature = wasmQuote.signature;
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
