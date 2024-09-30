use std::{fmt, str::FromStr};

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

impl From<PresentationDefinition> for WasmPresentationDefinition {
    fn from(value: PresentationDefinition) -> Self {
        Self { inner: value }
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
                    .map(|srs| srs.into_iter().map(|sr| sr.into()).collect()),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn purpose(&self) -> Option<String> {
        self.inner.purpose.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn input_descriptors(&self) -> Vec<WasmInputDescriptor> {
        self.inner
            .input_descriptors
            .iter()
            .cloned()
            .map(|i| i.into())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn submission_requirements(&self) -> Option<Vec<WasmSubmissionRequirement>> {
        self.inner
            .submission_requirements
            .as_ref()
            .map(|srs| srs.iter().cloned().map(|sr| sr.into()).collect())
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

impl From<InputDescriptor> for WasmInputDescriptor {
    fn from(value: InputDescriptor) -> Self {
        Self { inner: value }
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

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn purpose(&self) -> Option<String> {
        self.inner.purpose.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn constraints(&self) -> WasmConstraints {
        WasmConstraints {
            inner: self.inner.constraints.clone(),
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

impl From<Constraints> for WasmConstraints {
    fn from(value: Constraints) -> Self {
        Self { inner: value }
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

    #[wasm_bindgen(getter)]
    pub fn fields(&self) -> Vec<WasmField> {
        self.inner
            .fields
            .iter()
            .cloned()
            .map(|f| f.into())
            .collect()
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

impl From<Field> for WasmField {
    fn from(value: Field) -> Self {
        Self { inner: value }
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
                filter: filter.map(|f| f.into()),
                optional,
                predicate: predicate.map(|p| p.into()),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Vec<String> {
        self.inner.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn purpose(&self) -> Option<String> {
        self.inner.purpose.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn filter(&self) -> Option<WasmFilter> {
        self.inner
            .filter
            .as_ref()
            .map(|f| WasmFilter { inner: f.clone() })
    }

    #[wasm_bindgen(getter)]
    pub fn optional(&self) -> Option<bool> {
        self.inner.optional
    }

    #[wasm_bindgen(getter)]
    pub fn predicate(&self) -> Option<WasmOptionality> {
        self.inner
            .predicate
            .as_ref()
            .map(|p| WasmOptionality { inner: p.clone() })
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

impl From<Filter> for WasmFilter {
    fn from(value: Filter) -> Self {
        Self { inner: value }
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
                contains: contains.map(|c| Box::new(c.into())),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn r#type(&self) -> Option<String> {
        self.inner.r#type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn pattern(&self) -> Option<String> {
        self.inner.pattern.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn const_value(&self) -> Option<String> {
        self.inner.const_value.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn contains(&self) -> Option<WasmFilter> {
        self.inner
            .contains
            .as_ref()
            .map(|c| WasmFilter { inner: *c.clone() })
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

impl From<Optionality> for WasmOptionality {
    fn from(value: Optionality) -> Self {
        Self { inner: value }
    }
}

impl fmt::Display for WasmOptionality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self.inner {
            Optionality::Required => "required",
            Optionality::Preferred => "preferred",
        };
        write!(f, "{}", output)
    }
}

impl FromStr for WasmOptionality {
    type Err = Web5Error;

    fn from_str(s: &str) -> web5::errors::Result<Self> {
        let inner = match s {
            "required" => Optionality::Required,
            "preferred" => Optionality::Preferred,
            _ => return Err(Web5Error::Parameter(format!("unknown optionality {}", s))),
        };
        Ok(WasmOptionality { inner })
    }
}

#[wasm_bindgen]
impl WasmOptionality {
    #[wasm_bindgen(constructor)]
    pub fn new(optionality: &str) -> Result<WasmOptionality> {
        WasmOptionality::from_str(optionality).map_err(map_web5_err)
    }

    #[wasm_bindgen(getter)]
    pub fn optionality(&self) -> String {
        self.to_string()
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

impl From<SubmissionRequirement> for WasmSubmissionRequirement {
    fn from(value: SubmissionRequirement) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmSubmissionRequirement {
    #[allow(clippy::too_many_arguments)]
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
                from_nested: from_nested.map(|f| f.into_iter().map(|f| f.into()).collect()),
                name,
                purpose,
                count,
                min,
                max,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn rule(&self) -> WasmSubmissionRequirementRule {
        WasmSubmissionRequirementRule {
            inner: self.inner.rule.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn from(&self) -> Option<String> {
        self.inner.from.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn from_nested(&self) -> Option<Vec<WasmSubmissionRequirement>> {
        self.inner
            .from_nested
            .as_ref()
            .map(|fns| fns.iter().cloned().map(|fnr| fnr.into()).collect())
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn purpose(&self) -> Option<String> {
        self.inner.purpose.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn count(&self) -> Option<u32> {
        self.inner.count
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Option<u32> {
        self.inner.min
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Option<u32> {
        self.inner.max
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

impl From<SubmissionRequirementRule> for WasmSubmissionRequirementRule {
    fn from(value: SubmissionRequirementRule) -> Self {
        Self { inner: value }
    }
}

impl fmt::Display for WasmSubmissionRequirementRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self.inner {
            SubmissionRequirementRule::All => "all",
            SubmissionRequirementRule::Pick => "pick",
        };
        write!(f, "{}", output)
    }
}

impl FromStr for WasmSubmissionRequirementRule {
    type Err = Web5Error;

    fn from_str(s: &str) -> web5::errors::Result<Self> {
        let inner = match s {
            "all" => SubmissionRequirementRule::All,
            "pick" => SubmissionRequirementRule::Pick,
            _ => {
                return Err(Web5Error::Parameter(format!(
                    "unknown submission requirement rule {}",
                    s
                )))
            }
        };
        Ok(WasmSubmissionRequirementRule { inner })
    }
}

#[wasm_bindgen]
impl WasmSubmissionRequirementRule {
    #[wasm_bindgen(constructor)]
    pub fn new(rule: &str) -> Result<WasmSubmissionRequirementRule> {
        WasmSubmissionRequirementRule::from_str(rule).map_err(map_web5_err)
    }

    #[wasm_bindgen(getter)]
    pub fn rule(&self) -> String {
        self.to_string()
    }
}
