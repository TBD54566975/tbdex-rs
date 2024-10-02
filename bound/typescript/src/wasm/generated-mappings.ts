import wasm from "./";

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
