import { MessageMetadata } from ".";
import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import { Offering } from "../resources/offering";
import wasm from "../wasm";

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

  static fromJSONString = (json: string): Rfq => {
    const obj = JSON.parse(json);
    return new Rfq(obj.metadata, obj.data, obj.privateData, obj.signature);
  };

  toJSONString = (): string => {
    return JSON.stringify({
      metadata: this.metadata,
      data: this.data,
      privateData: this.privateData,
      signature: this.signature,
    });
  };

  static create = (
    to: string,
    from: string,
    createRfqData: CreateRfqData,
    protocol?: string,
    externalId?: string
  ): Rfq => {
    try {
      let json = wasm.rfq_create(
        to,
        from,
        JSON.stringify(createRfqData),
        protocol,
        externalId
      );
      let obj = JSON.parse(json);
      return new Rfq(obj.metadata, obj.data, obj.privateData, obj.signature);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  sign = (bearerDid: BearerDid) => {
    try {
      const signature = wasm.rfq_sign(this.toJSONString(), bearerDid.toWASM());
      this.signature = signature;
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verify = async () => {
    try {
      await wasm.rfq_verify(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verifyOfferingRequirements = async (offering: Offering) => {
    try {
      await wasm.rfq_verify_offering_requirements(
        this.toJSONString(),
        offering.toJSONString()
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verifyAllPrivateData = () => {
    try {
      wasm.rfq_verify_all_private_data(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };

  verifyPresentPrivateData = () => {
    try {
      wasm.rfq_verify_present_private_data(this.toJSONString());
    } catch (error) {
      throw tbdexError(error);
    }
  };
}

export type RfqData = {
  claimsHash?: string;
  offeringId: string;
  payin: SelectedPayinMethod;
  payout: SelectedPayoutMethod;
};

export type SelectedPayinMethod = {
  amount: string;
  kind: string;
  paymentDetailsHash?: string;
};

export type SelectedPayoutMethod = {
  kind: string;
  paymentDetailsHash?: string;
};

export type RfqPrivateData = {
  claims?: Array<any>;
  payin?: PrivatePaymentDetails;
  payout?: PrivatePaymentDetails;
  salt: string;
};

export type PrivatePaymentDetails = {
  paymentDetails?: any;
};

export type CreateRfqData = {
  claims: Array<any>;
  offeringId: string;
  payin: CreateSelectedPayinMethod;
  payout: CreateSelectedPayoutMethod;
};

export type CreateSelectedPayinMethod = {
  amount: string;
  kind: string;
  paymentDetails?: any;
};

export type CreateSelectedPayoutMethod = {
  kind: string;
  paymentDetails?: any;
};
