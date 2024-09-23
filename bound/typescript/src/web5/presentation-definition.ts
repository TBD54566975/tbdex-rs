import wasm from "../wasm";

export type PresentationDefinition = {
  id: string;
  name?: string;
  purpose?: string;
  input_descriptors: InputDescriptor[];
  submission_requirements?: SubmissionRequirement[];
};

export namespace PresentationDefinition {
  export const toWASM = (
    presentationDefinition: PresentationDefinition
  ): wasm.WasmPresentationDefinition => {
    return new wasm.WasmPresentationDefinition(
      presentationDefinition.id,
      presentationDefinition.name,
      presentationDefinition.purpose,
      presentationDefinition.input_descriptors.map(InputDescriptor.toWASM),
      presentationDefinition.submission_requirements?.map(
        SubmissionRequirement.toWASM
      )
    );
  };

  export const fromWASM = (
    wasmPresentationDefinition: wasm.WasmPresentationDefinition
  ): PresentationDefinition => {
    const presentationDefinition: PresentationDefinition = {
      id: wasmPresentationDefinition.id,
      input_descriptors: wasmPresentationDefinition.input_descriptors.map(
        InputDescriptor.fromWASM
      ),
    };

    if (wasmPresentationDefinition.name !== undefined)
      presentationDefinition.name = wasmPresentationDefinition.name;
    if (wasmPresentationDefinition.purpose !== undefined)
      presentationDefinition.purpose = wasmPresentationDefinition.purpose;
    if (wasmPresentationDefinition.submission_requirements !== undefined)
      presentationDefinition.submission_requirements =
        wasmPresentationDefinition.submission_requirements.map(
          SubmissionRequirement.fromWASM
        );

    return presentationDefinition;
  };
}

export type InputDescriptor = {
  id: string;
  name?: string;
  purpose?: string;
  constraints: Constraints;
};

export namespace InputDescriptor {
  export const toWASM = (
    inputDescriptor: InputDescriptor
  ): wasm.WasmInputDescriptor => {
    return new wasm.WasmInputDescriptor(
      inputDescriptor.id,
      inputDescriptor.name,
      inputDescriptor.purpose,
      Constraints.toWASM(inputDescriptor.constraints)
    );
  };

  export const fromWASM = (
    wasmInputDescriptor: wasm.WasmInputDescriptor
  ): InputDescriptor => {
    const inputDescriptor: InputDescriptor = {
      id: wasmInputDescriptor.id,
      constraints: Constraints.fromWASM(wasmInputDescriptor.constraints),
    };

    if (wasmInputDescriptor.name !== undefined)
      inputDescriptor.name = wasmInputDescriptor.name;
    if (wasmInputDescriptor.purpose !== undefined)
      inputDescriptor.purpose = wasmInputDescriptor.purpose;

    return inputDescriptor;
  };
}

export type Constraints = {
  fields: Field[];
};

export namespace Constraints {
  export const toWASM = (constraints: Constraints): wasm.WasmConstraints => {
    return new wasm.WasmConstraints(constraints.fields.map(Field.toWASM));
  };

  export const fromWASM = (
    wasmConstraints: wasm.WasmConstraints
  ): Constraints => {
    return {
      fields: wasmConstraints.fields.map(Field.fromWASM),
    };
  };
}

export type Field = {
  id?: string;
  name?: string;
  path: string[];
  purpose?: string;
  filter?: Filter;
  optional?: boolean;
  predicate?: Optionality;
};

export namespace Field {
  export const toWASM = (field: Field): wasm.WasmField => {
    return new wasm.WasmField(
      field.id,
      field.name,
      field.path,
      field.purpose,
      field.filter ? Filter.toWASM(field.filter) : undefined,
      field.optional,
      field.predicate ? Optionality.toWASM(field.predicate) : undefined
    );
  };

  export const fromWASM = (wasmField: wasm.WasmField): Field => {
    const field: Field = {
      path: wasmField.path,
    };

    if (wasmField.id !== undefined) field.id = wasmField.id;
    if (wasmField.name !== undefined) field.name = wasmField.name;
    if (wasmField.purpose !== undefined) field.purpose = wasmField.purpose;
    if (wasmField.filter !== undefined)
      field.filter = Filter.fromWASM(wasmField.filter);
    if (wasmField.optional !== undefined) field.optional = wasmField.optional;
    if (wasmField.predicate !== undefined)
      field.predicate = Optionality.fromWASM(wasmField.predicate);

    return field;
  };
}

export type Filter = {
  type?: string;
  pattern?: string;
  constValue?: string;
  contains?: Filter;
};

export namespace Filter {
  export const toWASM = (filter: Filter): wasm.WasmFilter => {
    return new wasm.WasmFilter(
      filter.type,
      filter.pattern,
      filter.constValue,
      filter.contains ? Filter.toWASM(filter.contains) : undefined
    );
  };

  export const fromWASM = (wasmFilter: wasm.WasmFilter): Filter => {
    const filter: Filter = {};

    if (wasmFilter.type !== undefined) filter.type = wasmFilter.type;
    if (wasmFilter.pattern !== undefined) filter.pattern = wasmFilter.pattern;
    if (wasmFilter.const_value !== undefined)
      filter.constValue = wasmFilter.const_value;
    if (wasmFilter.contains !== undefined)
      filter.contains = Filter.fromWASM(wasmFilter.contains);

    return filter;
  };
}

export type Optionality = {
  optionality: string;
};

export namespace Optionality {
  export const toWASM = (optionality: Optionality): wasm.WasmOptionality => {
    return new wasm.WasmOptionality(optionality.optionality);
  };

  export const fromWASM = (
    wasmOptionality: wasm.WasmOptionality
  ): Optionality => {
    return {
      optionality: wasmOptionality.optionality,
    };
  };
}

export type SubmissionRequirement = {
  rule: SubmissionRequirementRule;
  from?: string;
  from_nested?: SubmissionRequirement[];
  name?: string;
  purpose?: string;
  count?: number;
  min?: number;
  max?: number;
};

export namespace SubmissionRequirement {
  export const toWASM = (
    req: SubmissionRequirement
  ): wasm.WasmSubmissionRequirement => {
    return new wasm.WasmSubmissionRequirement(
      SubmissionRequirementRule.toWASM(req.rule),
      req.from,
      req.from_nested?.map(SubmissionRequirement.toWASM),
      req.name,
      req.purpose,
      req.count,
      req.min,
      req.max
    );
  };

  export const fromWASM = (
    wasmReq: wasm.WasmSubmissionRequirement
  ): SubmissionRequirement => {
    const submissionRequirement: SubmissionRequirement = {
      rule: SubmissionRequirementRule.fromWASM(wasmReq.rule),
    };

    if (wasmReq.from !== undefined) submissionRequirement.from = wasmReq.from;
    if (wasmReq.from_nested !== undefined)
      submissionRequirement.from_nested = wasmReq.from_nested.map(
        SubmissionRequirement.fromWASM
      );
    if (wasmReq.name !== undefined) submissionRequirement.name = wasmReq.name;
    if (wasmReq.purpose !== undefined)
      submissionRequirement.purpose = wasmReq.purpose;
    if (wasmReq.count !== undefined)
      submissionRequirement.count = wasmReq.count;
    if (wasmReq.min !== undefined) submissionRequirement.min = wasmReq.min;
    if (wasmReq.max !== undefined) submissionRequirement.max = wasmReq.max;

    return submissionRequirement;
  };
}

export type SubmissionRequirementRule = {
  rule: string;
};

export namespace SubmissionRequirementRule {
  export const toWASM = (
    rule: SubmissionRequirementRule
  ): wasm.WasmSubmissionRequirementRule => {
    return new wasm.WasmSubmissionRequirementRule(rule.rule);
  };

  export const fromWASM = (
    wasmRule: wasm.WasmSubmissionRequirementRule
  ): SubmissionRequirementRule => {
    return {
      rule: wasmRule.rule,
    };
  };
}
