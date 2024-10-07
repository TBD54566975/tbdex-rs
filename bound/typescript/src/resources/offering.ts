import { ResourceMetadata } from ".";
import { tbdexError } from "../errors";
import { PresentationDefinition } from "../credentials/presentation-definition";
import wasm from "../wasm";
import { BearerDid } from "../dids/bearer-did";

export class Offering {
  readonly metadata: ResourceMetadata;
  readonly data: OfferingData;
  signature: string;

  constructor(
    metadata: ResourceMetadata,
    data: OfferingData,
    signature: string
  ) {
    this.metadata = metadata;
    this.data = data;
    this.signature = signature;
  }

  static fromJSONString = (json: string): Offering => {
    const obj = JSON.parse(json);
    return new Offering(obj.metadata, obj.data, obj.signature);
  };

  toJSONString = (): string => {
    return JSON.stringify({
      metadata: this.metadata,
      data: this.data,
      signature: this.signature,
    });
  };

  static create = (
    from: string,
    data: OfferingData,
    protocol?: string
  ): Offering => {
    try {
      const json = wasm.offering_create(from, JSON.stringify(data), protocol);
      const obj = JSON.parse(json);
      return new Offering(obj.metadata, obj.data, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.offering_sign(
        this.toJSONString(),
        bearerDid.toWASM()
      );
      this.signature = signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = async () => {
    try {
      await wasm.offering_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type OfferingData = {
  cancellation: CancellationDetails;
  description: string;
  payin: PayinDetails;
  payout: PayoutDetails;
  payoutUnitsPerPayinUnit: string;
  requiredClaims?: PresentationDefinition;
};

export type CancellationDetails = {
  enabled: boolean;
  terms?: string;
  termsUrl?: string;
};

export type PayinDetails = {
  currencyCode: string;
  max?: string;
  methods: PayinMethod[];
  min?: string;
};

export type PayinMethod = {
  description?: string;
  fee?: string;
  group?: string;
  kind: string;
  max?: string;
  min?: string;
  name?: string;
  requiredPaymentDetails?: any;
};

export type PayoutDetails = {
  currencyCode: string;
  max?: string;
  methods: PayoutMethod[];
  min?: string;
};

export type PayoutMethod = {
  description?: string;
  estimatedSettlementTime: number;
  fee?: string;
  group?: string;
  kind: string;
  max?: string;
  min?: string;
  name?: string;
  requiredPaymentDetails?: any;
};
