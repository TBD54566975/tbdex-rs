import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import { PresentationDefinition } from "../presentation-definition";
import wasm from "../wasm";
import { ResourceMetadata } from "../wasm/generated-mappings";

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
    try {
      const object = JSON.parse(json);
      return new Offering(object.metadata, object.data, object.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toJSONString = (): string => {
    try {
      return JSON.stringify(this);
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
      const offering_data_json = JSON.stringify(data);
      const offering_json = wasm.offering_create(
        from,
        offering_data_json,
        protocol
      );
      const offering = JSON.parse(offering_json);
      return new Offering(offering.metadata, offering.data, offering.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const offering_json = JSON.stringify(this);
      const signature = wasm.offering_sign(offering_json, bearerDid.toWASM());
      this.signature = signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = async () => {
    try {
      const offering_json = JSON.stringify(this);
      await wasm.offering_verify(offering_json);
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
