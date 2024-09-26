import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import wasm from "../wasm";
import { OfferingData, ResourceMetadata } from "../wasm/mappings";

// TODO consider extending "Resource" class type
export class Offering {
  readonly metadata: ResourceMetadata;
  readonly data: OfferingData;
  readonly signature: string;

  constructor(
    metadata: ResourceMetadata,
    data: OfferingData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  private static fromWASM = (wasmOffering: wasm.WasmOffering): Offering => {
    try {
      return new Offering(
        ResourceMetadata.fromWASM(wasmOffering.metadata),
        OfferingData.fromWASM(wasmOffering.data),
        wasmOffering.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  private toWASM = (): wasm.WasmOffering => {
    try {
      return new wasm.WasmOffering(
        ResourceMetadata.toWASM(this.metadata),
        OfferingData.toWASM(this.data),
        this.signature
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  static fromJSONString = (json: string): Offering => {
    try {
      return Offering.fromWASM(wasm.WasmOffering.from_json_string(json));
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
    from: string,
    data: OfferingData,
    protocol?: string
  ): Offering => {
    try {
      return Offering.fromWASM(
        wasm.WasmOffering.create(from, OfferingData.toWASM(data), protocol)
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid): Offering => {
    try {
      const wasmOffering = this.toWASM();
      wasmOffering.sign(bearerDid.toWASM());

      return new Offering(this.metadata, this.data, wasmOffering.signature);
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
