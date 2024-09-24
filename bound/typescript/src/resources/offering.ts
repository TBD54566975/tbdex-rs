import { ResourceMetadata } from ".";
import { catchTbdexError } from "../errors";
import wasm from "../wasm";
import { PresentationDefinition } from "../presentation-definition";

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

  static fromWASM(wasmOffering: wasm.WasmOffering): Offering {
    return new Offering(
      ResourceMetadata.fromWASM(wasmOffering.metadata),
      OfferingData.fromWASM(wasmOffering.data),
      wasmOffering.signature
    );
  }

  toWASM(): wasm.WasmOffering {
    return new wasm.WasmOffering(
      ResourceMetadata.toWASM(this.metadata),
      OfferingData.toWASM(this.data),
      this.signature
    );
  }

  static fromJSONString(json: string): Offering {
    try {
      return Offering.fromWASM(wasm.WasmOffering.from_json_string(json));
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  toJSONString(): string {
    try {
      return this.toWASM().to_json_string();
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  static create(from: string, data: OfferingData, protocol?: string): Offering {
    try {
      return Offering.fromWASM(
        wasm.WasmOffering.create(from, OfferingData.toWASM(data), protocol)
      );
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  verify() {
    try {
      this.toWASM().verify();
    } catch (error) {
      throw catchTbdexError(error);
    }
  }
}

export type OfferingData = {
  description: string;
  payoutUnitsPerPayinUnit: string;
  payin: PayinDetails;
  payout: PayoutDetails;
  requiredClaims?: PresentationDefinition;
  cancellation: CancellationDetails;
};

export namespace OfferingData {
  export const fromWASM = (
    wasmOfferingData: wasm.WasmOfferingData
  ): OfferingData => {
    const payinDetails = PayinDetails.fromWASM(wasmOfferingData.payin);
    const payoutDetails = PayoutDetails.fromWASM(wasmOfferingData.payout);
    const cancellationDetails = CancellationDetails.fromWASM(
      wasmOfferingData.cancellation
    );

    const offeringData: OfferingData = {
      description: wasmOfferingData.description,
      payoutUnitsPerPayinUnit: wasmOfferingData.payout_units_per_payin_unit,
      payin: payinDetails,
      payout: payoutDetails,
      cancellation: cancellationDetails,
    };

    if (wasmOfferingData.required_claims !== undefined)
      offeringData.requiredClaims = PresentationDefinition.fromWASM(
        wasmOfferingData.required_claims
      );

    return offeringData;
  };

  export const toWASM = (offeringData: OfferingData): wasm.WasmOfferingData => {
    return new wasm.WasmOfferingData(
      offeringData.description,
      offeringData.payoutUnitsPerPayinUnit,
      PayinDetails.toWASM(offeringData.payin),
      PayoutDetails.toWASM(offeringData.payout),
      offeringData.requiredClaims
        ? PresentationDefinition.toWASM(offeringData.requiredClaims)
        : undefined,
      CancellationDetails.toWASM(offeringData.cancellation)
    );
  };
}

export type PayinDetails = {
  currencyCode: string;
  min?: string;
  max?: string;
  methods: PayinMethod[];
};

export namespace PayinDetails {
  export const fromWASM = (
    wasmPayinDetails: wasm.WasmPayinDetails
  ): PayinDetails => {
    const methods = wasmPayinDetails.methods.map(PayinMethod.fromWASM);

    const payinDetails: PayinDetails = {
      currencyCode: wasmPayinDetails.currency_code,
      methods: methods,
    };

    if (wasmPayinDetails.min !== undefined)
      payinDetails.min = wasmPayinDetails.min;
    if (wasmPayinDetails.max !== undefined)
      payinDetails.max = wasmPayinDetails.max;

    return payinDetails;
  };

  export const toWASM = (payinDetails: PayinDetails): wasm.WasmPayinDetails => {
    return new wasm.WasmPayinDetails(
      payinDetails.currencyCode,
      payinDetails.methods.map(PayinMethod.toWASM),
      payinDetails.min,
      payinDetails.max
    );
  };
}

export type PayinMethod = {
  kind: string;
  name?: string;
  description?: string;
  group?: string;
  requiredPaymentDetails?: any;
  fee?: string;
  min?: string;
  max?: string;
};

export namespace PayinMethod {
  const mapToObject = (map: Map<any, any>): any => {
    const obj: any = {};
    for (const [key, value] of map) {
      obj[key] = value instanceof Map ? mapToObject(value) : value;
    }
    return obj;
  };

  export const fromWASM = (wasmMethod: wasm.WasmPayinMethod): PayinMethod => {
    const method: PayinMethod = {
      kind: wasmMethod.kind,
      requiredPaymentDetails: mapToObject(wasmMethod.required_payment_details),
    };

    if (wasmMethod.name !== undefined) method.name = wasmMethod.name;
    if (wasmMethod.description !== undefined)
      method.description = wasmMethod.description;
    if (wasmMethod.group !== undefined) method.group = wasmMethod.group;
    if (wasmMethod.fee !== undefined) method.fee = wasmMethod.fee;
    if (wasmMethod.min !== undefined) method.min = wasmMethod.min;
    if (wasmMethod.max !== undefined) method.max = wasmMethod.max;

    return method;
  };

  export const toWASM = (method: PayinMethod): wasm.WasmPayinMethod => {
    return new wasm.WasmPayinMethod(
      method.kind,
      method.name,
      method.description,
      method.group,
      method.requiredPaymentDetails,
      method.fee,
      method.min,
      method.max
    );
  };
}

export type PayoutDetails = {
  currencyCode: string;
  min?: string;
  max?: string;
  methods: PayoutMethod[];
};

export namespace PayoutDetails {
  export const fromWASM = (
    wasmPayoutDetails: wasm.WasmPayoutDetails
  ): PayoutDetails => {
    const methods = wasmPayoutDetails.methods.map(PayoutMethod.fromWASM);

    const payoutDetails: PayoutDetails = {
      currencyCode: wasmPayoutDetails.currency_code,
      methods: methods,
    };

    if (wasmPayoutDetails.min !== undefined)
      payoutDetails.min = wasmPayoutDetails.min;
    if (wasmPayoutDetails.max !== undefined)
      payoutDetails.max = wasmPayoutDetails.max;

    return payoutDetails;
  };

  export const toWASM = (
    payoutDetails: PayoutDetails
  ): wasm.WasmPayoutDetails => {
    return new wasm.WasmPayoutDetails(
      payoutDetails.currencyCode,
      payoutDetails.methods.map(PayoutMethod.toWASM),
      payoutDetails.min,
      payoutDetails.max
    );
  };
}

export type PayoutMethod = {
  kind: string;
  estimatedSettlementTime: number;
  name?: string;
  description?: string;
  group?: string;
  requiredPaymentDetails?: any;
  fee?: string;
  min?: string;
  max?: string;
};

export namespace PayoutMethod {
  export const fromWASM = (wasmMethod: wasm.WasmPayoutMethod): PayoutMethod => {
    const method: PayoutMethod = {
      kind: wasmMethod.kind,
      estimatedSettlementTime: Number(wasmMethod.estimated_settlement_time),
    };

    if (wasmMethod.name !== undefined) method.name = wasmMethod.name;
    if (wasmMethod.description !== undefined)
      method.description = wasmMethod.description;
    if (wasmMethod.group !== undefined) method.group = wasmMethod.group;
    if (wasmMethod.fee !== undefined) method.fee = wasmMethod.fee;
    if (wasmMethod.min !== undefined) method.min = wasmMethod.min;
    if (wasmMethod.max !== undefined) method.max = wasmMethod.max;

    return method;
  };

  export const toWASM = (method: PayoutMethod): wasm.WasmPayoutMethod => {
    return new wasm.WasmPayoutMethod(
      method.kind,
      BigInt(method.estimatedSettlementTime),
      method.name,
      method.description,
      method.group,
      method.requiredPaymentDetails,
      method.fee,
      method.min,
      method.max
    );
  };
}

export type CancellationDetails = {
  enabled: boolean;
  termsUrl?: string;
  terms?: string;
};

export namespace CancellationDetails {
  export const fromWASM = (
    wasmCancellationDetails: wasm.WasmCancellationDetails
  ): CancellationDetails => {
    const cancellationDetails: CancellationDetails = {
      enabled: wasmCancellationDetails.enabled,
    };

    if (wasmCancellationDetails.terms_url !== undefined)
      cancellationDetails.termsUrl = wasmCancellationDetails.terms_url;
    if (wasmCancellationDetails.terms !== undefined)
      cancellationDetails.terms = wasmCancellationDetails.terms;

    return cancellationDetails;
  };

  export const toWASM = (
    cancellationDetails: CancellationDetails
  ): wasm.WasmCancellationDetails => {
    return new wasm.WasmCancellationDetails(
      cancellationDetails.enabled,
      cancellationDetails.termsUrl,
      cancellationDetails.terms
    );
  };
}
