import wasm from "../wasm";

export class PresentationDefinition {
  readonly id: string;
  readonly name?: string;
  readonly purpose?: string;
  readonly inputDescriptors: InputDescriptor[];
  readonly submissionRequirements?: SubmissionRequirement[];

  constructor(
    id: string,
    inputDescriptors: InputDescriptor[],
    name?: string,
    purpose?: string,
    submissionRequirements?: SubmissionRequirement[]
  ) {
    this.id = id;
    this.name = name;
    this.purpose = purpose;
    this.inputDescriptors = inputDescriptors;
    this.submissionRequirements = submissionRequirements;
  }

  static fromWASM(
    wasmDef: wasm.WasmPresentationDefinition
  ): PresentationDefinition {
    return new PresentationDefinition(
      wasmDef.id,
      wasmDef.input_descriptors.map(InputDescriptor.fromWASM),
      wasmDef.name,
      wasmDef.purpose,
      wasmDef.submission_requirements?.map(SubmissionRequirement.fromWASM)
    );
  }

  toWASM(): wasm.WasmPresentationDefinition {
    return new wasm.WasmPresentationDefinition(
      this.id,
      this.name,
      this.purpose,
      this.inputDescriptors.map((desc) => desc.toWASM()),
      this.submissionRequirements?.map((req) => req.toWASM())
    );
  }
}

export class InputDescriptor {
  readonly id: string;
  readonly name?: string;
  readonly purpose?: string;
  readonly constraints: Constraints;

  constructor(
    id: string,
    constraints: Constraints,
    name?: string,
    purpose?: string
  ) {
    this.id = id;
    this.name = name;
    this.purpose = purpose;
    this.constraints = constraints;
  }

  static fromWASM(wasmDesc: wasm.WasmInputDescriptor): InputDescriptor {
    return new InputDescriptor(
      wasmDesc.id,
      Constraints.fromWASM(wasmDesc.constraints),
      wasmDesc.name,
      wasmDesc.purpose
    );
  }

  toWASM(): wasm.WasmInputDescriptor {
    return new wasm.WasmInputDescriptor(
      this.id,
      this.name,
      this.purpose,
      this.constraints.toWASM()
    );
  }
}

export class Constraints {
  readonly fields: Field[];

  constructor(fields: Field[]) {
    this.fields = fields;
  }

  static fromWASM(wasmConstraints: wasm.WasmConstraints): Constraints {
    return new Constraints(wasmConstraints.fields.map(Field.fromWASM));
  }

  toWASM(): wasm.WasmConstraints {
    return new wasm.WasmConstraints(this.fields.map((field) => field.toWASM()));
  }
}

export class Field {
  readonly id?: string;
  readonly name?: string;
  readonly path: string[];
  readonly purpose?: string;
  readonly filter?: Filter;
  readonly optional?: boolean;
  readonly predicate?: Optionality;

  constructor(
    path: string[],
    id?: string,
    name?: string,
    purpose?: string,
    filter?: Filter,
    optional?: boolean,
    predicate?: Optionality
  ) {
    this.id = id;
    this.name = name;
    this.path = path;
    this.purpose = purpose;
    this.filter = filter;
    this.optional = optional;
    this.predicate = predicate;
  }

  static fromWASM(wasmField: wasm.WasmField): Field {
    return new Field(
      wasmField.path,
      wasmField.id,
      wasmField.name,
      wasmField.purpose,
      wasmField.filter ? Filter.fromWASM(wasmField.filter) : undefined,
      wasmField.optional,
      wasmField.predicate
        ? Optionality.fromWASM(wasmField.predicate)
        : undefined
    );
  }

  toWASM(): wasm.WasmField {
    return new wasm.WasmField(
      this.id,
      this.name,
      this.path,
      this.purpose,
      this.filter?.toWASM(),
      this.optional,
      this.predicate?.toWASM()
    );
  }
}

export class Filter {
  readonly type?: string;
  readonly pattern?: string;
  readonly constValue?: string;
  readonly contains?: Filter;

  constructor(
    type?: string,
    pattern?: string,
    constValue?: string,
    contains?: Filter
  ) {
    this.type = type;
    this.pattern = pattern;
    this.constValue = constValue;
    this.contains = contains;
  }

  static fromWASM(wasmFilter: wasm.WasmFilter): Filter {
    return new Filter(
      wasmFilter.type,
      wasmFilter.pattern,
      wasmFilter.const_value,
      wasmFilter.contains ? Filter.fromWASM(wasmFilter.contains) : undefined
    );
  }

  toWASM(): wasm.WasmFilter {
    return new wasm.WasmFilter(
      this.type,
      this.pattern,
      this.constValue,
      this.contains?.toWASM()
    );
  }
}

export class Optionality {
  readonly optionality: string;

  constructor(optionality: string) {
    this.optionality = optionality;
  }

  static fromWASM(wasmOptionality: wasm.WasmOptionality): Optionality {
    return new Optionality(wasmOptionality.optionality);
  }

  toWASM(): wasm.WasmOptionality {
    return new wasm.WasmOptionality(this.optionality);
  }
}

export class SubmissionRequirement {
  readonly rule: SubmissionRequirementRule;
  readonly from?: string;
  readonly fromNested?: SubmissionRequirement[];
  readonly name?: string;
  readonly purpose?: string;
  readonly count?: number;
  readonly min?: number;
  readonly max?: number;

  constructor(
    rule: SubmissionRequirementRule,
    from?: string,
    fromNested?: SubmissionRequirement[],
    name?: string,
    purpose?: string,
    count?: number,
    min?: number,
    max?: number
  ) {
    this.rule = rule;
    this.from = from;
    this.fromNested = fromNested;
    this.name = name;
    this.purpose = purpose;
    this.count = count;
    this.min = min;
    this.max = max;
  }

  static fromWASM(
    wasmReq: wasm.WasmSubmissionRequirement
  ): SubmissionRequirement {
    return new SubmissionRequirement(
      SubmissionRequirementRule.fromWASM(wasmReq.rule),
      wasmReq.from,
      wasmReq.from_nested?.map(SubmissionRequirement.fromWASM),
      wasmReq.name,
      wasmReq.purpose,
      wasmReq.count,
      wasmReq.min,
      wasmReq.max
    );
  }

  toWASM(): wasm.WasmSubmissionRequirement {
    return new wasm.WasmSubmissionRequirement(
      this.rule.toWASM(),
      this.from,
      this.fromNested?.map((req) => req.toWASM()),
      this.name,
      this.purpose,
      this.count,
      this.min,
      this.max
    );
  }
}

export class SubmissionRequirementRule {
  readonly rule: string;

  constructor(rule: string) {
    this.rule = rule;
  }

  static fromWASM(
    wasmRule: wasm.WasmSubmissionRequirementRule
  ): SubmissionRequirementRule {
    return new SubmissionRequirementRule(wasmRule.rule);
  }

  toWASM(): wasm.WasmSubmissionRequirementRule {
    return new wasm.WasmSubmissionRequirementRule(this.rule);
  }
}
