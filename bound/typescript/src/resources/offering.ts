import { catchTbdexError } from "../errors";
import wasm from "../wasm";

export class Offering {
  inner: wasm.WasmOffering;

  static create(from: string, data: OfferingData, protocol?: string): Offering {
    try {
      return new Offering(wasm.WasmOffering.create(from, data.toWasm(), protocol));
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  static fromJsonString(json: string): Offering {
    try {
      let inner = wasm.WasmOffering.from_json_string(json)
      return new Offering(inner);
    } catch (error) {
      throw catchTbdexError(error);
    }
  }

  verify() {
    try {
      this.inner.verify()
    } catch (error) {
      throw catchTbdexError(error)
    }
  }

  // TODO replace with static fromWasm()
  constructor(inner: wasm.WasmOffering) {
    this.inner = inner;
  }
}

export class OfferingData {
  description: string;
  payoutUnitsPerPayinUnit: string;
  payin: PayinDetails;
  payout: PayoutDetails;
  requiredClaims?: any;
  cancellation: CancellationDetails;

  constructor(
    description: string,
    payoutUnitsPerPayinUnit: string,
    payin: PayinDetails,
    payout: PayoutDetails,
    requiredClaims: any | undefined,
    cancellation: CancellationDetails
  ) {
    this.description = description;
    this.payoutUnitsPerPayinUnit = payoutUnitsPerPayinUnit;
    this.payin = payin;
    this.payout = payout;
    this.requiredClaims = requiredClaims;
    this.cancellation = cancellation;
  }

  toWasm(): wasm.WasmOfferingData {
    return new wasm.WasmOfferingData(
      this.description,
      this.payoutUnitsPerPayinUnit,
      this.payin.toWasm(),
      this.payout.toWasm(),
      this.requiredClaims,
      this.cancellation.toWasm()
    );
  }
}

export class PayinDetails {
  currencyCode: string;
  min?: string;
  max?: string;
  methods: PayinMethod[];

  constructor(currencyCode: string, methods: PayinMethod[], min?: string, max?: string) {
    this.currencyCode = currencyCode;
    this.min = min;
    this.max = max;
    this.methods = methods;
  }

  toWasm(): wasm.WasmPayinDetails {
    return new wasm.WasmPayinDetails(
      this.currencyCode,
      this.methods.map(method => method.toWasm()),
      this.min,
      this.max,
    );
  }
}

export class PayinMethod {
  kind: string;
  name?: string;
  description?: string;
  group?: string;
  requiredPaymentDetails?: any;
  fee?: string;
  min?: string;
  max?: string;

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

  toWasm(): wasm.WasmPayinMethod {
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
  currencyCode: string;
  min?: string;
  max?: string;
  methods: PayoutMethod[];

  constructor(currencyCode: string, methods: PayoutMethod[], min?: string, max?: string) {
    this.currencyCode = currencyCode;
    this.min = min;
    this.max = max;
    this.methods = methods;
  }

  toWasm(): wasm.WasmPayoutDetails {
    return new wasm.WasmPayoutDetails(
      this.currencyCode,
      this.methods.map(method => method.toWasm()),
      this.min,
      this.max,
    );
  }
}

export class PayoutMethod {
  kind: string;
  name?: string;
  description?: string;
  group?: string;
  requiredPaymentDetails?: any;
  fee?: string;
  min?: string;
  max?: string;
  estimatedSettlementTime: number;

  constructor(
    kind: string,
    estimatedSettlementTime: number,
    name?: string,
    description?: string,
    group?: string,
    requiredPaymentDetails?: any,
    fee?: string,
    min?: string,
    max?: string,
  ) {
    this.kind = kind;
    this.name = name;
    this.description = description;
    this.group = group;
    this.requiredPaymentDetails = requiredPaymentDetails;
    this.fee = fee;
    this.min = min;
    this.max = max;
    this.estimatedSettlementTime = estimatedSettlementTime;
  }

  toWasm(): wasm.WasmPayoutMethod {
    return new wasm.WasmPayoutMethod(
      this.kind,
      BigInt(this.estimatedSettlementTime),
      this.name,
      this.description,
      this.group,
      this.requiredPaymentDetails,
      this.fee,
      this.min,
      this.max,
    );
  }
}

export class CancellationDetails {
  enabled: boolean;
  termsUrl?: string;
  terms?: string;

  constructor(enabled: boolean, termsUrl?: string, terms?: string) {
    this.enabled = enabled;
    this.termsUrl = termsUrl;
    this.terms = terms;
  }

  toWasm(): wasm.WasmCancellationDetails {
    return new wasm.WasmCancellationDetails(this.enabled, this.termsUrl, this.terms);
  }
}