export type PresentationDefinition = {
  id: string;
  input_descriptors: InputDescriptor[];
  name?: string;
  purpose?: string;
  submission_requirements?: SubmissionRequirement[];
};

export type InputDescriptor = {
  constraints: Constraints;
  id: string;
  name?: string;
  purpose?: string;
};

export type Constraints = {
  fields: Field[];
};

export type Field = {
  filter?: Filter;
  id?: string;
  name?: string;
  optional?: boolean;
  path: string[];
  predicate?: Optionality;
  purpose?: string;
};

export type Filter = {
  const?: string;
  contains?: Filter;
  pattern?: string;
  type?: string;
};

export type Optionality = "required" | "preferred";

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

export type SubmissionRequirementRule = "all" | "pick";
