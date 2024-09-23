import { ResourceMetadata } from ".";
import { catchTbdexError } from "../errors";
import wasm from "../wasm";
import { PresentationDefinition } from "../web5/presentation-definition";

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
      this.metadata.toWASM(),
      this.data.toWASM(),
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
        wasm.WasmOffering.create(from, data.toWASM(), protocol)
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

// TODO consider using type's instead of class's like we used to
export class OfferingData {
  readonly description: string;
  readonly payoutUnitsPerPayinUnit: string;
  readonly payin: PayinDetails;
  readonly payout: PayoutDetails;
  readonly requiredClaims?: PresentationDefinition;
  readonly cancellation: CancellationDetails;

  constructor(
    description: string,
    payoutUnitsPerPayinUnit: string,
    payin: PayinDetails,
    payout: PayoutDetails,
    cancellation: CancellationDetails,
    requiredClaims?: PresentationDefinition
  ) {
    this.description = description;
    this.payoutUnitsPerPayinUnit = payoutUnitsPerPayinUnit;
    this.payin = payin;
    this.payout = payout;
    this.cancellation = cancellation;
    this.requiredClaims = requiredClaims;
  }

  static fromWASM(wasmData: wasm.WasmOfferingData): OfferingData {
    return new OfferingData(
      wasmData.description,
      wasmData.payout_units_per_payin_unit,
      PayinDetails.fromWASM(wasmData.payin),
      PayoutDetails.fromWASM(wasmData.payout),
      CancellationDetails.fromWASM(wasmData.cancellation),
      wasmData.required_claims
        ? PresentationDefinition.fromWASM(wasmData.required_claims)
        : undefined
    );
  }

  toWASM(): wasm.WasmOfferingData {
    return new wasm.WasmOfferingData(
      this.description,
      this.payoutUnitsPerPayinUnit,
      this.payin.toWASM(),
      this.payout.toWASM(),
      this.requiredClaims?.toWASM(),
      this.cancellation.toWASM()
    );
  }
}

export class PayinDetails {
  readonly currencyCode: string;
  readonly min?: string;
  readonly max?: string;
  readonly methods: PayinMethod[];

  constructor(
    currencyCode: string,
    methods: PayinMethod[],
    min?: string,
    max?: string
  ) {
    this.currencyCode = currencyCode;
    this.methods = methods;
    this.min = min;
    this.max = max;
  }

  static fromWASM(wasmPayin: wasm.WasmPayinDetails): PayinDetails {
    return new PayinDetails(
      wasmPayin.currency_code,
      wasmPayin.methods.map(PayinMethod.fromWASM),
      wasmPayin.min,
      wasmPayin.max
    );
  }

  toWASM(): wasm.WasmPayinDetails {
    return new wasm.WasmPayinDetails(
      this.currencyCode,
      this.methods.map((method) => method.toWASM()),
      this.min,
      this.max
    );
  }
}

export class PayinMethod {
  readonly kind: string;
  readonly name?: string;
  readonly description?: string;
  readonly group?: string;
  readonly requiredPaymentDetails?: any;
  readonly fee?: string;
  readonly min?: string;
  readonly max?: string;

  constructor(
    kind: string,
    name?: string,
    description?: string,
    group?: string,
    requiredPaymentDetails?: any,
    fee?: string,
    min?: string,
    max?: string
  ) {
    this.kind = kind;
    this.name = name;
    this.description = description;
    this.group = group;
    this.requiredPaymentDetails = requiredPaymentDetails;
    this.fee = fee;
    this.min = min;
    this.max = max;
  }

  static fromWASM(wasmMethod: wasm.WasmPayinMethod): PayinMethod {
    return new PayinMethod(
      wasmMethod.kind,
      wasmMethod.name,
      wasmMethod.description,
      wasmMethod.group,
      wasmMethod.required_payment_details,
      wasmMethod.fee,
      wasmMethod.min,
      wasmMethod.max
    );
  }

  toWASM(): wasm.WasmPayinMethod {
    return new wasm.WasmPayinMethod(
      this.kind,
      this.name,
      this.description,
      this.group,
      this.requiredPaymentDetails,
      this.fee,
      this.min,
      this.max
    );
  }
}

export class PayoutDetails {
  readonly currencyCode: string;
  readonly min?: string;
  readonly max?: string;
  readonly methods: PayoutMethod[];

  constructor(
    currencyCode: string,
    methods: PayoutMethod[],
    min?: string,
    max?: string
  ) {
    this.currencyCode = currencyCode;
    this.methods = methods;
    this.min = min;
    this.max = max;
  }

  static fromWASM(wasmPayout: wasm.WasmPayoutDetails): PayoutDetails {
    return new PayoutDetails(
      wasmPayout.currency_code,
      wasmPayout.methods.map(PayoutMethod.fromWASM),
      wasmPayout.min,
      wasmPayout.max
    );
  }

  toWASM(): wasm.WasmPayoutDetails {
    return new wasm.WasmPayoutDetails(
      this.currencyCode,
      this.methods.map((method) => method.toWASM()),
      this.min,
      this.max
    );
  }
}

export class PayoutMethod {
  readonly kind: string;
  readonly name?: string;
  readonly description?: string;
  readonly group?: string;
  readonly requiredPaymentDetails?: any;
  readonly fee?: string;
  readonly min?: string;
  readonly max?: string;
  readonly estimatedSettlementTime: number;

  constructor(
    kind: string,
    estimatedSettlementTime: number,
    name?: string,
    description?: string,
    group?: string,
    requiredPaymentDetails?: any,
    fee?: string,
    min?: string,
    max?: string
  ) {
    this.kind = kind;
    this.estimatedSettlementTime = estimatedSettlementTime;
    this.name = name;
    this.description = description;
    this.group = group;
    this.requiredPaymentDetails = requiredPaymentDetails;
    this.fee = fee;
    this.min = min;
    this.max = max;
  }

  static fromWASM(wasmMethod: wasm.WasmPayoutMethod): PayoutMethod {
    return new PayoutMethod(
      wasmMethod.kind,
      Number(wasmMethod.estimated_settlement_time),
      wasmMethod.name,
      wasmMethod.description,
      wasmMethod.group,
      wasmMethod.required_payment_details,
      wasmMethod.fee,
      wasmMethod.min,
      wasmMethod.max
    );
  }

  toWASM(): wasm.WasmPayoutMethod {
    return new wasm.WasmPayoutMethod(
      this.kind,
      BigInt(this.estimatedSettlementTime),
      this.name,
      this.description,
      this.group,
      this.requiredPaymentDetails,
      this.fee,
      this.min,
      this.max
    );
  }
}

export class CancellationDetails {
  readonly enabled: boolean;
  readonly termsUrl?: string;
  readonly terms?: string;

  constructor(enabled: boolean, termsUrl?: string, terms?: string) {
    this.enabled = enabled;
    this.termsUrl = termsUrl;
    this.terms = terms;
  }

  static fromWASM(
    wasmCancellation: wasm.WasmCancellationDetails
  ): CancellationDetails {
    return new CancellationDetails(
      wasmCancellation.enabled,
      wasmCancellation.terms_url,
      wasmCancellation.terms
    );
  }

  toWASM(): wasm.WasmCancellationDetails {
    return new wasm.WasmCancellationDetails(
      this.enabled,
      this.termsUrl,
      this.terms
    );
  }
}
