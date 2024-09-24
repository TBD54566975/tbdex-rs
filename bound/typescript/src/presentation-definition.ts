import wasm from "./wasm";

export type CancellationDetails = {
  enabled: boolean;
  terms?: string;
  termsUrl?: string;
};

export namespace CancellationDetails {
  export const toWASM = (
    obj: CancellationDetails
  ): wasm.WasmCancellationDetails => {
    return new wasm.WasmCancellationDetails(
      obj.enabled,
      obj.termsUrl,
      obj.terms
    );
  };

  export const fromWASM = (
    obj: wasm.WasmCancellationDetails
  ): CancellationDetails => {
    const result: CancellationDetails = {
      enabled: obj.enabled,
    };

    if (obj.terms !== undefined) result.terms = obj.terms;

    if (obj.terms_url !== undefined) result.termsUrl = obj.terms_url;

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
      obj.predicate ? Optionality.toWASM(obj.predicate) : undefined
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
      obj.contains ? Filter.toWASM(obj.contains) : undefined
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
      Constraints.toWASM(obj.constraints)
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
export type Offering = {
  data: OfferingData;
  metadata: ResourceMetadata;
  signature: string;
};

export namespace Offering {
  export const toWASM = (obj: Offering): wasm.WasmOffering => {
    return new wasm.WasmOffering(
      ResourceMetadata.toWASM(obj.metadata),
      OfferingData.toWASM(obj.data),
      obj.signature
    );
  };

  export const fromWASM = (obj: wasm.WasmOffering): Offering => {
    const result: Offering = {
      data: OfferingData.fromWASM(obj.data),
      metadata: ResourceMetadata.fromWASM(obj.metadata),
      signature: obj.signature,
    };

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
      CancellationDetails.toWASM(obj.cancellation)
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
        obj.required_claims
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
      obj.max
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
  requiredPaymentDetails: any;
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
      obj.max
    );
  };

  export const fromWASM = (obj: wasm.WasmPayinMethod): PayinMethod => {
    const result: PayinMethod = {
      kind: obj.kind,
      requiredPaymentDetails: obj.required_payment_details,
    };

    if (obj.description !== undefined) result.description = obj.description;

    if (obj.fee !== undefined) result.fee = obj.fee;

    if (obj.group !== undefined) result.group = obj.group;

    if (obj.max !== undefined) result.max = obj.max;

    if (obj.min !== undefined) result.min = obj.min;

    if (obj.name !== undefined) result.name = obj.name;

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
      obj.max
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
  estimatedSettlementTime: bigint;
  fee?: string;
  group?: string;
  kind: string;
  max?: string;
  min?: string;
  name?: string;
  requiredPaymentDetails: any;
};

export namespace PayoutMethod {
  export const toWASM = (obj: PayoutMethod): wasm.WasmPayoutMethod => {
    return new wasm.WasmPayoutMethod(
      obj.kind,
      obj.estimatedSettlementTime,
      obj.name,
      obj.description,
      obj.group,
      obj.requiredPaymentDetails,
      obj.fee,
      obj.min,
      obj.max
    );
  };

  export const fromWASM = (obj: wasm.WasmPayoutMethod): PayoutMethod => {
    const result: PayoutMethod = {
      estimatedSettlementTime: obj.estimated_settlement_time,
      kind: obj.kind,
      requiredPaymentDetails: obj.required_payment_details,
    };

    if (obj.description !== undefined) result.description = obj.description;

    if (obj.fee !== undefined) result.fee = obj.fee;

    if (obj.group !== undefined) result.group = obj.group;

    if (obj.max !== undefined) result.max = obj.max;

    if (obj.min !== undefined) result.min = obj.min;

    if (obj.name !== undefined) result.name = obj.name;

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
    obj: PresentationDefinition
  ): wasm.WasmPresentationDefinition => {
    return new wasm.WasmPresentationDefinition(
      obj.id,
      obj.name,
      obj.purpose,
      obj.input_descriptors?.map(InputDescriptor.toWASM),
      obj.submission_requirements?.map(SubmissionRequirement.toWASM)
    );
  };

  export const fromWASM = (
    obj: wasm.WasmPresentationDefinition
  ): PresentationDefinition => {
    const result: PresentationDefinition = {
      id: obj.id,
      input_descriptors: obj.input_descriptors?.map(InputDescriptor.fromWASM),
    };

    if (obj.name !== undefined) result.name = obj.name;

    if (obj.purpose !== undefined) result.purpose = obj.purpose;

    if (obj.submission_requirements !== undefined)
      result.submission_requirements = obj.submission_requirements?.map(
        SubmissionRequirement.fromWASM
      );

    return result;
  };
}
export type ResourceKind = {
  kind: string;
};

export namespace ResourceKind {
  export const toWASM = (obj: ResourceKind): wasm.WasmResourceKind => {
    return new wasm.WasmResourceKind(obj.kind);
  };

  export const fromWASM = (obj: wasm.WasmResourceKind): ResourceKind => {
    const result: ResourceKind = {
      kind: obj.kind,
    };

    return result;
  };
}
export type ResourceMetadata = {
  createdAt: string;
  from: string;
  id: string;
  kind: ResourceKind;
  protocol: string;
  updatedAt?: string;
};

export namespace ResourceMetadata {
  export const toWASM = (obj: ResourceMetadata): wasm.WasmResourceMetadata => {
    return new wasm.WasmResourceMetadata(
      ResourceKind.toWASM(obj.kind),
      obj.from,
      obj.id,
      obj.protocol,
      obj.createdAt,
      obj.updatedAt
    );
  };

  export const fromWASM = (
    obj: wasm.WasmResourceMetadata
  ): ResourceMetadata => {
    const result: ResourceMetadata = {
      createdAt: obj.created_at,
      from: obj.from,
      id: obj.id,
      kind: ResourceKind.fromWASM(obj.kind),
      protocol: obj.protocol,
    };

    if (obj.updated_at !== undefined) result.updatedAt = obj.updated_at;

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
    obj: SubmissionRequirement
  ): wasm.WasmSubmissionRequirement => {
    return new wasm.WasmSubmissionRequirement(
      SubmissionRequirementRule.toWASM(obj.rule),
      obj.from,
      obj.from_nested?.map(SubmissionRequirement.toWASM),
      obj.name,
      obj.purpose,
      obj.count,
      obj.min,
      obj.max
    );
  };

  export const fromWASM = (
    obj: wasm.WasmSubmissionRequirement
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
    obj: SubmissionRequirementRule
  ): wasm.WasmSubmissionRequirementRule => {
    return new wasm.WasmSubmissionRequirementRule(obj.rule);
  };

  export const fromWASM = (
    obj: wasm.WasmSubmissionRequirementRule
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
