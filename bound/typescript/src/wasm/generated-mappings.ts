import wasm from "./";

export type BalanceData = {
  available: string;
  currencyCode: string;
};

export namespace BalanceData {
  export const toWASM = (obj: BalanceData): wasm.WasmBalanceData => {
    return new wasm.WasmBalanceData(obj.currencyCode, obj.available);
  };

  export const fromWASM = (obj: wasm.WasmBalanceData): BalanceData => {
    const result: BalanceData = {
      available: obj.available,
      currencyCode: obj.currency_code,
    };

    return result;
  };
}

export type CancelData = {
  reason?: string;
};

export namespace CancelData {
  export const toWASM = (obj: CancelData): wasm.WasmCancelData => {
    return new wasm.WasmCancelData(obj.reason);
  };

  export const fromWASM = (obj: wasm.WasmCancelData): CancelData => {
    const result: CancelData = {};

    if (obj.reason !== undefined) result.reason = obj.reason;

    return result;
  };
}

export type CancellationDetails = {
  enabled: boolean;
  terms?: string;
  termsUrl?: string;
};

export namespace CancellationDetails {
  export const toWASM = (
    obj: CancellationDetails,
  ): wasm.WasmCancellationDetails => {
    return new wasm.WasmCancellationDetails(
      obj.enabled,
      obj.termsUrl,
      obj.terms,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmCancellationDetails,
  ): CancellationDetails => {
    const result: CancellationDetails = {
      enabled: obj.enabled,
    };

    if (obj.terms !== undefined) result.terms = obj.terms;
    if (obj.terms_url !== undefined) result.termsUrl = obj.terms_url;

    return result;
  };
}

export type CloseData = {
  reason?: string;
  success?: boolean;
};

export namespace CloseData {
  export const toWASM = (obj: CloseData): wasm.WasmCloseData => {
    return new wasm.WasmCloseData(obj.reason, obj.success);
  };

  export const fromWASM = (obj: wasm.WasmCloseData): CloseData => {
    const result: CloseData = {};

    if (obj.reason !== undefined) result.reason = obj.reason;
    if (obj.success !== undefined) result.success = obj.success;

    return result;
  };
}

export type Constraints = {
  fields: Field[];
};

export namespace Constraints {
  export const toWASM = (obj: Constraints): wasm.WasmConstraints => {
    return new wasm.WasmConstraints(obj.fields?.map(Field.toWASM));
  };

  export const fromWASM = (obj: wasm.WasmConstraints): Constraints => {
    const result: Constraints = {
      fields: obj.fields?.map(Field.fromWASM),
    };

    return result;
  };
}

export type CreateRfqData = {
  claims: Array<any>;
  offeringId: string;
  payin: CreateSelectedPayinMethod;
  payout: CreateSelectedPayoutMethod;
};

export namespace CreateRfqData {
  export const toWASM = (obj: CreateRfqData): wasm.WasmCreateRfqData => {
    return new wasm.WasmCreateRfqData(
      obj.offeringId,
      CreateSelectedPayinMethod.toWASM(obj.payin),
      CreateSelectedPayoutMethod.toWASM(obj.payout),
      obj.claims,
    );
  };

  export const fromWASM = (obj: wasm.WasmCreateRfqData): CreateRfqData => {
    const result: CreateRfqData = {
      claims: obj.claims,
      offeringId: obj.offering_id,
      payin: CreateSelectedPayinMethod.fromWASM(obj.payin),
      payout: CreateSelectedPayoutMethod.fromWASM(obj.payout),
    };

    return result;
  };
}

export type CreateSelectedPayinMethod = {
  amount: string;
  kind: string;
  paymentDetails?: any;
};

export namespace CreateSelectedPayinMethod {
  export const toWASM = (
    obj: CreateSelectedPayinMethod,
  ): wasm.WasmCreateSelectedPayinMethod => {
    return new wasm.WasmCreateSelectedPayinMethod(
      obj.kind,
      obj.paymentDetails,
      obj.amount,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmCreateSelectedPayinMethod,
  ): CreateSelectedPayinMethod => {
    const result: CreateSelectedPayinMethod = {
      amount: obj.amount,
      kind: obj.kind,
    };

    if (obj.payment_details !== undefined)
      result.paymentDetails = obj.payment_details;

    return result;
  };
}

export type CreateSelectedPayoutMethod = {
  kind: string;
  paymentDetails?: any;
};

export namespace CreateSelectedPayoutMethod {
  export const toWASM = (
    obj: CreateSelectedPayoutMethod,
  ): wasm.WasmCreateSelectedPayoutMethod => {
    return new wasm.WasmCreateSelectedPayoutMethod(
      obj.kind,
      obj.paymentDetails,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmCreateSelectedPayoutMethod,
  ): CreateSelectedPayoutMethod => {
    const result: CreateSelectedPayoutMethod = {
      kind: obj.kind,
    };

    if (obj.payment_details !== undefined)
      result.paymentDetails = obj.payment_details;

    return result;
  };
}

export type Did = {
  fragment?: string;
  id: string;
  method: string;
  params?: any;
  path?: string;
  query?: string;
  uri: string;
  url: string;
};

export namespace Did {
  export const toWASM = (obj: Did): wasm.WasmDid => {
    return new wasm.WasmDid(
      obj.uri,
      obj.url,
      obj.method,
      obj.id,
      obj.params,
      obj.path,
      obj.query,
      obj.fragment,
    );
  };

  export const fromWASM = (obj: wasm.WasmDid): Did => {
    const result: Did = {
      id: obj.id,
      method: obj.method,
      uri: obj.uri,
      url: obj.url,
    };

    if (obj.fragment !== undefined) result.fragment = obj.fragment;
    if (obj.params !== undefined) result.params = obj.params;
    if (obj.path !== undefined) result.path = obj.path;
    if (obj.query !== undefined) result.query = obj.query;

    return result;
  };
}

export type Document = {
  alsoKnownAs?: string[];
  assertionMethod?: string[];
  authentication?: string[];
  capabilityDelegation?: string[];
  capabilityInvocation?: string[];
  context?: string[];
  controller?: string[];
  id: string;
  keyAgreement?: string[];
  service?: Service[];
  verificationMethod: VerificationMethod[];
};

export namespace Document {
  export const toWASM = (obj: Document): wasm.WasmDocument => {
    return new wasm.WasmDocument(
      obj.id,
      obj.context,
      obj.controller,
      obj.alsoKnownAs,
      obj.verificationMethod?.map(VerificationMethod.toWASM),
      obj.authentication,
      obj.assertionMethod,
      obj.keyAgreement,
      obj.capabilityInvocation,
      obj.capabilityDelegation,
      obj.service?.map(Service.toWASM),
    );
  };

  export const fromWASM = (obj: wasm.WasmDocument): Document => {
    const result: Document = {
      id: obj.id,
      verificationMethod: obj.verification_method?.map(
        VerificationMethod.fromWASM,
      ),
    };

    if (obj.also_known_as !== undefined) result.alsoKnownAs = obj.also_known_as;
    if (obj.assertion_method !== undefined)
      result.assertionMethod = obj.assertion_method;
    if (obj.authentication !== undefined)
      result.authentication = obj.authentication;
    if (obj.capability_delegation !== undefined)
      result.capabilityDelegation = obj.capability_delegation;
    if (obj.capability_invocation !== undefined)
      result.capabilityInvocation = obj.capability_invocation;
    if (obj.context !== undefined) result.context = obj.context;
    if (obj.controller !== undefined) result.controller = obj.controller;
    if (obj.key_agreement !== undefined)
      result.keyAgreement = obj.key_agreement;
    if (obj.service !== undefined)
      result.service = obj.service?.map(Service.fromWASM);

    return result;
  };
}

export type FetchOptions = {
  body?: Uint8Array;
  headers?: any;
  method?: string;
};

export namespace FetchOptions {
  export const toWASM = (obj: FetchOptions): wasm.WasmFetchOptions => {
    return new wasm.WasmFetchOptions(obj.method, obj.headers, obj.body);
  };

  export const fromWASM = (obj: wasm.WasmFetchOptions): FetchOptions => {
    const result: FetchOptions = {};

    if (obj.body !== undefined) result.body = obj.body;
    if (obj.headers !== undefined) result.headers = obj.headers;
    if (obj.method !== undefined) result.method = obj.method;

    return result;
  };
}

export type Field = {
  filter?: Filter;
  id?: string;
  name?: string;
  optional?: boolean;
  path: string[];
  predicate?: Optionality;
  purpose?: string;
};

export namespace Field {
  export const toWASM = (obj: Field): wasm.WasmField => {
    return new wasm.WasmField(
      obj.id,
      obj.name,
      obj.path,
      obj.purpose,
      obj.filter ? Filter.toWASM(obj.filter) : undefined,
      obj.optional,
      obj.predicate ? Optionality.toWASM(obj.predicate) : undefined,
    );
  };

  export const fromWASM = (obj: wasm.WasmField): Field => {
    const result: Field = {
      path: obj.path,
    };

    if (obj.filter !== undefined) result.filter = Filter.fromWASM(obj.filter);
    if (obj.id !== undefined) result.id = obj.id;
    if (obj.name !== undefined) result.name = obj.name;
    if (obj.optional !== undefined) result.optional = obj.optional;
    if (obj.predicate !== undefined)
      result.predicate = Optionality.fromWASM(obj.predicate);
    if (obj.purpose !== undefined) result.purpose = obj.purpose;

    return result;
  };
}

export type Filter = {
  const?: string;
  contains?: Filter;
  pattern?: string;
  type?: string;
};

export namespace Filter {
  export const toWASM = (obj: Filter): wasm.WasmFilter => {
    return new wasm.WasmFilter(
      obj.type,
      obj.pattern,
      obj.const,
      obj.contains ? Filter.toWASM(obj.contains) : undefined,
    );
  };

  export const fromWASM = (obj: wasm.WasmFilter): Filter => {
    const result: Filter = {};

    if (obj.const_value !== undefined) result.const = obj.const_value;
    if (obj.contains !== undefined)
      result.contains = Filter.fromWASM(obj.contains);
    if (obj.pattern !== undefined) result.pattern = obj.pattern;
    if (obj.type !== undefined) result.type = obj.type;

    return result;
  };
}

export type InputDescriptor = {
  constraints: Constraints;
  id: string;
  name?: string;
  purpose?: string;
};

export namespace InputDescriptor {
  export const toWASM = (obj: InputDescriptor): wasm.WasmInputDescriptor => {
    return new wasm.WasmInputDescriptor(
      obj.id,
      obj.name,
      obj.purpose,
      Constraints.toWASM(obj.constraints),
    );
  };

  export const fromWASM = (obj: wasm.WasmInputDescriptor): InputDescriptor => {
    const result: InputDescriptor = {
      constraints: Constraints.fromWASM(obj.constraints),
      id: obj.id,
    };

    if (obj.name !== undefined) result.name = obj.name;
    if (obj.purpose !== undefined) result.purpose = obj.purpose;

    return result;
  };
}

export type JsonSerializedMessage = {
  json: string;
  kind: string;
};

export namespace JsonSerializedMessage {
  export const toWASM = (
    obj: JsonSerializedMessage,
  ): wasm.WasmJsonSerializedMessage => {
    return new wasm.WasmJsonSerializedMessage();
  };

  export const fromWASM = (
    obj: wasm.WasmJsonSerializedMessage,
  ): JsonSerializedMessage => {
    const result: JsonSerializedMessage = {
      json: obj.json,
      kind: obj.kind,
    };

    return result;
  };
}

export type Jwk = {
  alg?: string;
  crv: string;
  d?: string;
  kty: string;
  x: string;
  y?: string;
};

export namespace Jwk {
  export const toWASM = (obj: Jwk): wasm.WasmJwk => {
    return new wasm.WasmJwk(obj.alg, obj.kty, obj.crv, obj.d, obj.x, obj.y);
  };

  export const fromWASM = (obj: wasm.WasmJwk): Jwk => {
    const result: Jwk = {
      crv: obj.crv,
      kty: obj.kty,
      x: obj.x,
    };

    if (obj.alg !== undefined) result.alg = obj.alg;
    if (obj.d !== undefined) result.d = obj.d;
    if (obj.y !== undefined) result.y = obj.y;

    return result;
  };
}

export type MessageMetadata = {
  createdAt: string;
  exchangeId: string;
  externalId?: string;
  from: string;
  id: string;
  kind: string;
  protocol: string;
  to: string;
};

export namespace MessageMetadata {
  export const toWASM = (obj: MessageMetadata): wasm.WasmMessageMetadata => {
    return new wasm.WasmMessageMetadata(
      obj.from,
      obj.to,
      obj.kind,
      obj.id,
      obj.exchangeId,
      obj.externalId,
      obj.protocol,
      obj.createdAt,
    );
  };

  export const fromWASM = (obj: wasm.WasmMessageMetadata): MessageMetadata => {
    const result: MessageMetadata = {
      createdAt: obj.created_at,
      exchangeId: obj.exchange_id,
      from: obj.from,
      id: obj.id,
      kind: obj.kind,
      protocol: obj.protocol,
      to: obj.to,
    };

    if (obj.external_id !== undefined) result.externalId = obj.external_id;

    return result;
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

export namespace OfferingData {
  export const toWASM = (obj: OfferingData): wasm.WasmOfferingData => {
    return new wasm.WasmOfferingData(
      obj.description,
      obj.payoutUnitsPerPayinUnit,
      PayinDetails.toWASM(obj.payin),
      PayoutDetails.toWASM(obj.payout),
      obj.requiredClaims
        ? PresentationDefinition.toWASM(obj.requiredClaims)
        : undefined,
      CancellationDetails.toWASM(obj.cancellation),
    );
  };

  export const fromWASM = (obj: wasm.WasmOfferingData): OfferingData => {
    const result: OfferingData = {
      cancellation: CancellationDetails.fromWASM(obj.cancellation),
      description: obj.description,
      payin: PayinDetails.fromWASM(obj.payin),
      payout: PayoutDetails.fromWASM(obj.payout),
      payoutUnitsPerPayinUnit: obj.payout_units_per_payin_unit,
    };

    if (obj.required_claims !== undefined)
      result.requiredClaims = PresentationDefinition.fromWASM(
        obj.required_claims,
      );

    return result;
  };
}

export type Optionality = {
  optionality: string;
};

export namespace Optionality {
  export const toWASM = (obj: Optionality): wasm.WasmOptionality => {
    return new wasm.WasmOptionality(obj.optionality);
  };

  export const fromWASM = (obj: wasm.WasmOptionality): Optionality => {
    const result: Optionality = {
      optionality: obj.optionality,
    };

    return result;
  };
}

export type OrderData = {};

export namespace OrderData {
  export const toWASM = (obj: OrderData): wasm.WasmOrderData => {
    return new wasm.WasmOrderData();
  };

  export const fromWASM = (obj: wasm.WasmOrderData): OrderData => {
    const result: OrderData = {};

    return result;
  };
}

export type OrderInstructionsData = {
  payin: PaymentInstruction;
  payout: PaymentInstruction;
};

export namespace OrderInstructionsData {
  export const toWASM = (
    obj: OrderInstructionsData,
  ): wasm.WasmOrderInstructionsData => {
    return new wasm.WasmOrderInstructionsData(
      PaymentInstruction.toWASM(obj.payin),
      PaymentInstruction.toWASM(obj.payout),
    );
  };

  export const fromWASM = (
    obj: wasm.WasmOrderInstructionsData,
  ): OrderInstructionsData => {
    const result: OrderInstructionsData = {
      payin: PaymentInstruction.fromWASM(obj.payin),
      payout: PaymentInstruction.fromWASM(obj.payout),
    };

    return result;
  };
}

export type OrderStatusData = {
  details?: string;
  status: string;
};

export namespace OrderStatusData {
  export const toWASM = (obj: OrderStatusData): wasm.WasmOrderStatusData => {
    return new wasm.WasmOrderStatusData(obj.status, obj.details);
  };

  export const fromWASM = (obj: wasm.WasmOrderStatusData): OrderStatusData => {
    const result: OrderStatusData = {
      status: obj.status,
    };

    if (obj.details !== undefined) result.details = obj.details;

    return result;
  };
}

export type PayinDetails = {
  currencyCode: string;
  max?: string;
  methods: PayinMethod[];
  min?: string;
};

export namespace PayinDetails {
  export const toWASM = (obj: PayinDetails): wasm.WasmPayinDetails => {
    return new wasm.WasmPayinDetails(
      obj.currencyCode,
      obj.methods?.map(PayinMethod.toWASM),
      obj.min,
      obj.max,
    );
  };

  export const fromWASM = (obj: wasm.WasmPayinDetails): PayinDetails => {
    const result: PayinDetails = {
      currencyCode: obj.currency_code,
      methods: obj.methods?.map(PayinMethod.fromWASM),
    };

    if (obj.max !== undefined) result.max = obj.max;
    if (obj.min !== undefined) result.min = obj.min;

    return result;
  };
}

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

export namespace PayinMethod {
  export const toWASM = (obj: PayinMethod): wasm.WasmPayinMethod => {
    return new wasm.WasmPayinMethod(
      obj.kind,
      obj.name,
      obj.description,
      obj.group,
      obj.requiredPaymentDetails,
      obj.fee,
      obj.min,
      obj.max,
    );
  };

  export const fromWASM = (obj: wasm.WasmPayinMethod): PayinMethod => {
    const result: PayinMethod = {
      kind: obj.kind,
    };

    if (obj.description !== undefined) result.description = obj.description;
    if (obj.fee !== undefined) result.fee = obj.fee;
    if (obj.group !== undefined) result.group = obj.group;
    if (obj.max !== undefined) result.max = obj.max;
    if (obj.min !== undefined) result.min = obj.min;
    if (obj.name !== undefined) result.name = obj.name;
    if (obj.required_payment_details !== undefined)
      result.requiredPaymentDetails = obj.required_payment_details;

    return result;
  };
}

export type PaymentInstruction = {
  instruction?: string;
  link?: string;
};

export namespace PaymentInstruction {
  export const toWASM = (
    obj: PaymentInstruction,
  ): wasm.WasmPaymentInstruction => {
    return new wasm.WasmPaymentInstruction(obj.link, obj.instruction);
  };

  export const fromWASM = (
    obj: wasm.WasmPaymentInstruction,
  ): PaymentInstruction => {
    const result: PaymentInstruction = {};

    if (obj.instruction !== undefined) result.instruction = obj.instruction;
    if (obj.link !== undefined) result.link = obj.link;

    return result;
  };
}

export type PayoutDetails = {
  currencyCode: string;
  max?: string;
  methods: PayoutMethod[];
  min?: string;
};

export namespace PayoutDetails {
  export const toWASM = (obj: PayoutDetails): wasm.WasmPayoutDetails => {
    return new wasm.WasmPayoutDetails(
      obj.currencyCode,
      obj.methods?.map(PayoutMethod.toWASM),
      obj.min,
      obj.max,
    );
  };

  export const fromWASM = (obj: wasm.WasmPayoutDetails): PayoutDetails => {
    const result: PayoutDetails = {
      currencyCode: obj.currency_code,
      methods: obj.methods?.map(PayoutMethod.fromWASM),
    };

    if (obj.max !== undefined) result.max = obj.max;
    if (obj.min !== undefined) result.min = obj.min;

    return result;
  };
}

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

export namespace PayoutMethod {
  export const toWASM = (obj: PayoutMethod): wasm.WasmPayoutMethod => {
    return new wasm.WasmPayoutMethod(
      obj.kind,
      BigInt(obj.estimatedSettlementTime),
      obj.name,
      obj.description,
      obj.group,
      obj.requiredPaymentDetails,
      obj.fee,
      obj.min,
      obj.max,
    );
  };

  export const fromWASM = (obj: wasm.WasmPayoutMethod): PayoutMethod => {
    const result: PayoutMethod = {
      estimatedSettlementTime: Number(obj.estimated_settlement_time),
      kind: obj.kind,
    };

    if (obj.description !== undefined) result.description = obj.description;
    if (obj.fee !== undefined) result.fee = obj.fee;
    if (obj.group !== undefined) result.group = obj.group;
    if (obj.max !== undefined) result.max = obj.max;
    if (obj.min !== undefined) result.min = obj.min;
    if (obj.name !== undefined) result.name = obj.name;
    if (obj.required_payment_details !== undefined)
      result.requiredPaymentDetails = obj.required_payment_details;

    return result;
  };
}

export type PresentationDefinition = {
  id: string;
  input_descriptors: InputDescriptor[];
  name?: string;
  purpose?: string;
  submission_requirements?: SubmissionRequirement[];
};

export namespace PresentationDefinition {
  export const toWASM = (
    obj: PresentationDefinition,
  ): wasm.WasmPresentationDefinition => {
    return new wasm.WasmPresentationDefinition(
      obj.id,
      obj.name,
      obj.purpose,
      obj.input_descriptors?.map(InputDescriptor.toWASM),
      obj.submission_requirements?.map(SubmissionRequirement.toWASM),
    );
  };

  export const fromWASM = (
    obj: wasm.WasmPresentationDefinition,
  ): PresentationDefinition => {
    const result: PresentationDefinition = {
      id: obj.id,
      input_descriptors: obj.input_descriptors?.map(InputDescriptor.fromWASM),
    };

    if (obj.name !== undefined) result.name = obj.name;
    if (obj.purpose !== undefined) result.purpose = obj.purpose;
    if (obj.submission_requirements !== undefined)
      result.submission_requirements = obj.submission_requirements?.map(
        SubmissionRequirement.fromWASM,
      );

    return result;
  };
}

export type PrivatePaymentDetails = {
  paymentDetails?: any;
};

export namespace PrivatePaymentDetails {
  export const toWASM = (
    obj: PrivatePaymentDetails,
  ): wasm.WasmPrivatePaymentDetails => {
    return new wasm.WasmPrivatePaymentDetails(obj.paymentDetails);
  };

  export const fromWASM = (
    obj: wasm.WasmPrivatePaymentDetails,
  ): PrivatePaymentDetails => {
    const result: PrivatePaymentDetails = {};

    if (obj.payment_details !== undefined)
      result.paymentDetails = obj.payment_details;

    return result;
  };
}

export type QuoteData = {
  expiresAt: string;
  payin: QuoteDetails;
  payout: QuoteDetails;
  payoutUnitsPerPayinUnit: string;
};

export namespace QuoteData {
  export const toWASM = (obj: QuoteData): wasm.WasmQuoteData => {
    return new wasm.WasmQuoteData(
      obj.expiresAt,
      obj.payoutUnitsPerPayinUnit,
      QuoteDetails.toWASM(obj.payin),
      QuoteDetails.toWASM(obj.payout),
    );
  };

  export const fromWASM = (obj: wasm.WasmQuoteData): QuoteData => {
    const result: QuoteData = {
      expiresAt: obj.expires_at,
      payin: QuoteDetails.fromWASM(obj.payin),
      payout: QuoteDetails.fromWASM(obj.payout),
      payoutUnitsPerPayinUnit: obj.payout_units_per_payin_unit,
    };

    return result;
  };
}

export type QuoteDetails = {
  currencyCode: string;
  fee?: string;
  subtotal: string;
  total: string;
};

export namespace QuoteDetails {
  export const toWASM = (obj: QuoteDetails): wasm.WasmQuoteDetails => {
    return new wasm.WasmQuoteDetails(
      obj.currencyCode,
      obj.subtotal,
      obj.total,
      obj.fee,
    );
  };

  export const fromWASM = (obj: wasm.WasmQuoteDetails): QuoteDetails => {
    const result: QuoteDetails = {
      currencyCode: obj.currency_code,
      subtotal: obj.subtotal,
      total: obj.total,
    };

    if (obj.fee !== undefined) result.fee = obj.fee;

    return result;
  };
}

export type ResourceMetadata = {
  createdAt: string;
  from: string;
  id: string;
  kind: string;
  protocol: string;
  updatedAt?: string;
};

export namespace ResourceMetadata {
  export const toWASM = (obj: ResourceMetadata): wasm.WasmResourceMetadata => {
    return new wasm.WasmResourceMetadata(
      obj.kind,
      obj.from,
      obj.id,
      obj.protocol,
      obj.createdAt,
      obj.updatedAt,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmResourceMetadata,
  ): ResourceMetadata => {
    const result: ResourceMetadata = {
      createdAt: obj.created_at,
      from: obj.from,
      id: obj.id,
      kind: obj.kind,
      protocol: obj.protocol,
    };

    if (obj.updated_at !== undefined) result.updatedAt = obj.updated_at;

    return result;
  };
}

export type Response = {
  body: Uint8Array;
  headers?: any;
  statusCode: number;
};

export namespace Response {
  export const toWASM = (obj: Response): wasm.WasmResponse => {
    return new wasm.WasmResponse(obj.statusCode, obj.headers, obj.body);
  };

  export const fromWASM = (obj: wasm.WasmResponse): Response => {
    const result: Response = {
      body: obj.body,
      statusCode: obj.status_code,
    };

    if (obj.headers !== undefined) result.headers = obj.headers;

    return result;
  };
}

export type RfqData = {
  claimsHash?: string;
  offeringId: string;
  payin: SelectedPayinMethod;
  payout: SelectedPayoutMethod;
};

export namespace RfqData {
  export const toWASM = (obj: RfqData): wasm.WasmRfqData => {
    return new wasm.WasmRfqData(
      obj.offeringId,
      SelectedPayinMethod.toWASM(obj.payin),
      SelectedPayoutMethod.toWASM(obj.payout),
      obj.claimsHash,
    );
  };

  export const fromWASM = (obj: wasm.WasmRfqData): RfqData => {
    const result: RfqData = {
      offeringId: obj.offering_id,
      payin: SelectedPayinMethod.fromWASM(obj.payin),
      payout: SelectedPayoutMethod.fromWASM(obj.payout),
    };

    if (obj.claims_hash !== undefined) result.claimsHash = obj.claims_hash;

    return result;
  };
}

export type RfqPrivateData = {
  claims?: Array<any>;
  payin?: PrivatePaymentDetails;
  payout?: PrivatePaymentDetails;
  salt: string;
};

export namespace RfqPrivateData {
  export const toWASM = (obj: RfqPrivateData): wasm.WasmRfqPrivateData => {
    return new wasm.WasmRfqPrivateData(
      obj.salt,
      obj.payin ? PrivatePaymentDetails.toWASM(obj.payin) : undefined,
      obj.payout ? PrivatePaymentDetails.toWASM(obj.payout) : undefined,
      obj.claims,
    );
  };

  export const fromWASM = (obj: wasm.WasmRfqPrivateData): RfqPrivateData => {
    const result: RfqPrivateData = {
      salt: obj.salt,
    };

    if (obj.claims !== undefined) result.claims = obj.claims;
    if (obj.payin !== undefined)
      result.payin = PrivatePaymentDetails.fromWASM(obj.payin);
    if (obj.payout !== undefined)
      result.payout = PrivatePaymentDetails.fromWASM(obj.payout);

    return result;
  };
}

export type SelectedPayinMethod = {
  amount: string;
  kind: string;
  paymentDetailsHash?: string;
};

export namespace SelectedPayinMethod {
  export const toWASM = (
    obj: SelectedPayinMethod,
  ): wasm.WasmSelectedPayinMethod => {
    return new wasm.WasmSelectedPayinMethod(
      obj.kind,
      obj.paymentDetailsHash,
      obj.amount,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmSelectedPayinMethod,
  ): SelectedPayinMethod => {
    const result: SelectedPayinMethod = {
      amount: obj.amount,
      kind: obj.kind,
    };

    if (obj.payment_details_hash !== undefined)
      result.paymentDetailsHash = obj.payment_details_hash;

    return result;
  };
}

export type SelectedPayoutMethod = {
  kind: string;
  paymentDetailsHash?: string;
};

export namespace SelectedPayoutMethod {
  export const toWASM = (
    obj: SelectedPayoutMethod,
  ): wasm.WasmSelectedPayoutMethod => {
    return new wasm.WasmSelectedPayoutMethod(obj.kind, obj.paymentDetailsHash);
  };

  export const fromWASM = (
    obj: wasm.WasmSelectedPayoutMethod,
  ): SelectedPayoutMethod => {
    const result: SelectedPayoutMethod = {
      kind: obj.kind,
    };

    if (obj.payment_details_hash !== undefined)
      result.paymentDetailsHash = obj.payment_details_hash;

    return result;
  };
}

export type Service = {
  id: string;
  serviceEndpoint: string[];
  type: string;
};

export namespace Service {
  export const toWASM = (obj: Service): wasm.WasmService => {
    return new wasm.WasmService(obj.id, obj.type, obj.serviceEndpoint);
  };

  export const fromWASM = (obj: wasm.WasmService): Service => {
    const result: Service = {
      id: obj.id,
      serviceEndpoint: obj.service_endpoint,
      type: obj.type,
    };

    return result;
  };
}

export type SubmissionRequirement = {
  count?: number;
  from?: string;
  from_nested?: SubmissionRequirement[];
  max?: number;
  min?: number;
  name?: string;
  purpose?: string;
  rule: SubmissionRequirementRule;
};

export namespace SubmissionRequirement {
  export const toWASM = (
    obj: SubmissionRequirement,
  ): wasm.WasmSubmissionRequirement => {
    return new wasm.WasmSubmissionRequirement(
      SubmissionRequirementRule.toWASM(obj.rule),
      obj.from,
      obj.from_nested?.map(SubmissionRequirement.toWASM),
      obj.name,
      obj.purpose,
      obj.count,
      obj.min,
      obj.max,
    );
  };

  export const fromWASM = (
    obj: wasm.WasmSubmissionRequirement,
  ): SubmissionRequirement => {
    const result: SubmissionRequirement = {
      rule: SubmissionRequirementRule.fromWASM(obj.rule),
    };

    if (obj.count !== undefined) result.count = obj.count;
    if (obj.from !== undefined) result.from = obj.from;
    if (obj.from_nested !== undefined)
      result.from_nested = obj.from_nested?.map(SubmissionRequirement.fromWASM);
    if (obj.max !== undefined) result.max = obj.max;
    if (obj.min !== undefined) result.min = obj.min;
    if (obj.name !== undefined) result.name = obj.name;
    if (obj.purpose !== undefined) result.purpose = obj.purpose;

    return result;
  };
}

export type SubmissionRequirementRule = {
  rule: string;
};

export namespace SubmissionRequirementRule {
  export const toWASM = (
    obj: SubmissionRequirementRule,
  ): wasm.WasmSubmissionRequirementRule => {
    return new wasm.WasmSubmissionRequirementRule(obj.rule);
  };

  export const fromWASM = (
    obj: wasm.WasmSubmissionRequirementRule,
  ): SubmissionRequirementRule => {
    const result: SubmissionRequirementRule = {
      rule: obj.rule,
    };

    return result;
  };
}

export type TbdexError = {
  isWeb5Error: boolean;
  message: string;
  variant: string;
};

export namespace TbdexError {
  export const toWASM = (obj: TbdexError): wasm.WasmTbdexError => {
    return new wasm.WasmTbdexError();
  };

  export const fromWASM = (obj: wasm.WasmTbdexError): TbdexError => {
    const result: TbdexError = {
      isWeb5Error: obj.is_web5_error,
      message: obj.message,
      variant: obj.variant,
    };

    return result;
  };
}

export type VerificationMethod = {
  controller: string;
  id: string;
  publicKeyJwk: Jwk;
  type: string;
};

export namespace VerificationMethod {
  export const toWASM = (
    obj: VerificationMethod,
  ): wasm.WasmVerificationMethod => {
    return new wasm.WasmVerificationMethod(
      obj.id,
      obj.type,
      obj.controller,
      Jwk.toWASM(obj.publicKeyJwk),
    );
  };

  export const fromWASM = (
    obj: wasm.WasmVerificationMethod,
  ): VerificationMethod => {
    const result: VerificationMethod = {
      controller: obj.controller,
      id: obj.id,
      publicKeyJwk: Jwk.fromWASM(obj.public_key_jwk),
      type: obj.type,
    };

    return result;
  };
}
