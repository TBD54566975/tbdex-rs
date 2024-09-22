use crate::errors::{map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    credentials::presentation_definition::{
        Constraints, Field, Filter, InputDescriptor, Optionality, PresentationDefinition,
        SubmissionRequirement, SubmissionRequirementRule,
    },
    errors::Web5Error,
};

#[wasm_bindgen]
pub struct WasmPresentationDefinition {
    inner: PresentationDefinition,
}

impl From<WasmPresentationDefinition> for PresentationDefinition {
    fn from(value: WasmPresentationDefinition) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPresentationDefinition {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        name: Option<String>,
        purpose: Option<String>,
        input_descriptors: Vec<WasmInputDescriptor>,
        submission_requirements: Option<Vec<WasmSubmissionRequirement>>,
    ) -> Self {
        Self {
            inner: PresentationDefinition {
                id,
                name,
                purpose,
                input_descriptors: input_descriptors.into_iter().map(|i| i.into()).collect(),
                submission_requirements: submission_requirements
                    .and_then(|srs| Some(srs.into_iter().map(|sr| sr.into()).collect())),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmInputDescriptor {
    inner: InputDescriptor,
}

impl From<WasmInputDescriptor> for InputDescriptor {
    fn from(value: WasmInputDescriptor) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmInputDescriptor {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        name: Option<String>,
        purpose: Option<String>,
        constraints: WasmConstraints,
    ) -> Self {
        Self {
            inner: InputDescriptor {
                id,
                name,
                purpose,
                constraints: constraints.into(),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmConstraints {
    inner: Constraints,
}

impl From<WasmConstraints> for Constraints {
    fn from(value: WasmConstraints) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmConstraints {
    #[wasm_bindgen(constructor)]
    pub fn new(fields: Vec<WasmField>) -> Self {
        Self {
            inner: Constraints {
                fields: fields.into_iter().map(|f| f.into()).collect(),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmField {
    inner: Field,
}

impl From<WasmField> for Field {
    fn from(value: WasmField) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmField {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        path: Vec<String>,
        purpose: Option<String>,
        filter: Option<WasmFilter>,
        optional: Option<bool>,
        predicate: Option<WasmOptionality>,
    ) -> Self {
        Self {
            inner: Field {
                id,
                name,
                path,
                purpose,
                filter: filter.and_then(|f| Some(f.into())),
                optional,
                predicate: predicate.and_then(|p| Some(p.into())),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmFilter {
    inner: Filter,
}

impl From<WasmFilter> for Filter {
    fn from(value: WasmFilter) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        r#type: Option<String>,
        pattern: Option<String>,
        const_value: Option<String>,
        contains: Option<WasmFilter>,
    ) -> Self {
        Self {
            inner: Filter {
                r#type,
                pattern,
                const_value,
                contains: contains.and_then(|c| Some(Box::new(c.into()))),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmOptionality {
    inner: Optionality,
}

impl From<WasmOptionality> for Optionality {
    fn from(value: WasmOptionality) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmOptionality {
    #[wasm_bindgen(constructor)]
    pub fn new(optionality: &str) -> Result<WasmOptionality> {
        Ok(Self {
            inner: match optionality {
                "required" => Optionality::Required,
                "preferred" => Optionality::Preferred,
                _ => {
                    return Err(map_web5_err(Web5Error::Parameter(format!(
                        "unknown optionality {}",
                        optionality
                    ))))
                }
            },
        })
    }
}

#[wasm_bindgen]
pub struct WasmSubmissionRequirement {
    inner: SubmissionRequirement,
}

impl From<WasmSubmissionRequirement> for SubmissionRequirement {
    fn from(value: WasmSubmissionRequirement) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmSubmissionRequirement {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rule: WasmSubmissionRequirementRule,
        from: Option<String>,
        from_nested: Option<Vec<WasmSubmissionRequirement>>,
        name: Option<String>,
        purpose: Option<String>,
        count: Option<u32>,
        min: Option<u32>,
        max: Option<u32>,
    ) -> Self {
        Self {
            inner: SubmissionRequirement {
                rule: rule.into(),
                from,
                from_nested: from_nested
                    .and_then(|f| Some(f.into_iter().map(|f| f.into()).collect())),
                name,
                purpose,
                count,
                min,
                max,
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmSubmissionRequirementRule {
    inner: SubmissionRequirementRule,
}

impl From<WasmSubmissionRequirementRule> for SubmissionRequirementRule {
    fn from(value: WasmSubmissionRequirementRule) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmSubmissionRequirementRule {
    #[wasm_bindgen(constructor)]
    pub fn new(rule: &str) -> Result<WasmSubmissionRequirementRule> {
        Ok(Self {
            inner: match rule {
                "all" => SubmissionRequirementRule::All,
                "pick" => SubmissionRequirementRule::Pick,
                _ => {
                    return Err(map_web5_err(Web5Error::Parameter(format!(
                        "unknown submission requirement rule {}",
                        rule
                    ))))
                }
            },
        })
    }
}
