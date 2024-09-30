import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import { Offering } from "../resources/offering";
import wasm from "../wasm";
import {
  MessageMetadata,
  RfqData,
  RfqPrivateData,
  CreateRfqData,
} from "../wasm/generated-mappings";

export class Rfq {
  readonly metadata: MessageMetadata;
  readonly data: RfqData;
  readonly privateData?: RfqPrivateData;
  signature: string;

  constructor(
    metadata: MessageMetadata,
    data: RfqData,
    privateData: RfqPrivateData | undefined,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.privateData = privateData;
    this.signature = signature;
  }

  static fromWASM = (wasmRfq: wasm.WasmRfq): Rfq => {
    try {
      const privateData = wasmRfq.private_data
        ? RfqPrivateData.fromWASM(wasmRfq.private_data)
        : undefined;

      return new Rfq(
        MessageMetadata.fromWASM(wasmRfq.metadata),
        RfqData.fromWASM(wasmRfq.data),
        privateData,
        wasmRfq.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmRfq => {
    try {
      const wasmPrivateData = this.privateData
        ? RfqPrivateData.toWASM(this.privateData)
        : undefined;

      return new wasm.WasmRfq(
        MessageMetadata.toWASM(this.metadata),
        RfqData.toWASM(this.data),
        wasmPrivateData,
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Rfq => {
    try {
      return Rfq.fromWASM(wasm.WasmRfq.from_json_string(json));
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
    createRfqData: CreateRfqData,
    protocol?: string,
    externalId?: string
  ): Rfq => {
    try {
      return Rfq.fromWASM(
        wasm.WasmRfq.create(
          to,
          from,
          CreateRfqData.toWASM(createRfqData),
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
      const wasmRfq = this.toWASM();
      wasmRfq.sign(bearerDid.toWASM());
      this.signature = wasmRfq.signature;
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

  verifyOfferingRequirements = (offering: Offering) => {
    try {
      this.toWASM().verify_offering_requirements(offering.toWASM());
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verifyAllPrivateData = () => {
    try {
      this.toWASM().verify_all_private_data();
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verifyPresentPrivateData = () => {
    try {
      this.toWASM().verify_present_private_data();
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
