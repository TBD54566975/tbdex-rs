use super::Resource;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::resources::{offering::Offering as InnerOffering, Resource as InnerResource};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Offering(pub Arc<RwLock<InnerOffering>>);

impl Offering {
    pub fn new(from: String, data: data::OfferingData, protocol: String) -> Result<Self> {
        let offering =
            InnerOffering::new(from, data.to_inner()?, protocol).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(offering))))
    }

    pub fn get_data(&self) -> Result<data::Offering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(data::Offering {
            metadata: inner_offering.metadata.clone(),
            data: data::OfferingData::from_inner(inner_offering.data.clone())?,
            signature: inner_offering.signature.clone(),
        })
    }

    pub fn to_inner(&self) -> Result<InnerOffering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(inner_offering.clone())
    }
}

impl Resource for Offering {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut offering = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        offering
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let offering = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        offering.verify().map_err(|e| Arc::new(e.into()))
    }
}

pub mod data {
    use super::*;
    use tbdex::resources::{
        offering::{
            OfferingData as InnerOfferingData, PayinDetails as InnerPayinDetails,
            PayinMethod as InnerPayinMethod, PayoutDetails as InnerPayoutDetails,
            PayoutMethod as InnerPayoutMethod,
        },
        ResourceMetadata,
    };
    use web5::apid::credentials::presentation_definition::PresentationDefinition;

    #[derive(Clone)]
    pub struct Offering {
        pub metadata: ResourceMetadata,
        pub data: OfferingData,
        pub signature: String,
    }

    #[derive(Clone)]
    pub struct OfferingData {
        pub description: String,
        pub payout_units_per_payin_unit: String,
        pub payin: PayinDetails,
        pub payout: PayoutDetails,
        pub required_claims: PresentationDefinition,
    }

    impl OfferingData {
        pub fn to_inner(&self) -> Result<InnerOfferingData> {
            Ok(InnerOfferingData {
                description: self.description.clone(),
                payout_units_per_payin_unit: self.payout_units_per_payin_unit.clone(),
                payin: self.payin.to_inner()?,
                payout: self.payout.to_inner()?,
                required_claims: self.required_claims.clone(),
            })
        }

        pub fn from_inner(inner: InnerOfferingData) -> Result<Self> {
            Ok(Self {
                description: inner.description.clone(),
                payout_units_per_payin_unit: inner.payout_units_per_payin_unit.clone(),
                payin: PayinDetails::from_inner(inner.payin.clone())?,
                payout: PayoutDetails::from_inner(inner.payout.clone())?,
                required_claims: inner.required_claims.clone(),
            })
        }
    }

    #[derive(Clone)]
    pub struct PayinDetails {
        pub currency_code: String,
        pub min: Option<String>,
        pub max: Option<String>,
        pub methods: Vec<PayinMethod>,
    }

    impl PayinDetails {
        pub fn to_inner(&self) -> Result<InnerPayinDetails> {
            let methods: Result<Vec<InnerPayinMethod>> = self
                .methods
                .clone()
                .into_iter()
                .map(|m| m.to_inner())
                .collect();

            Ok(InnerPayinDetails {
                currency_code: self.currency_code.clone(),
                min: self.min.clone(),
                max: self.max.clone(),
                methods: methods?,
            })
        }

        pub fn from_inner(inner: InnerPayinDetails) -> Result<Self> {
            let methods: Result<Vec<PayinMethod>> = inner
                .methods
                .clone()
                .into_iter()
                .map(PayinMethod::from_inner)
                .collect();

            Ok(Self {
                currency_code: inner.currency_code.clone(),
                min: inner.min.clone(),
                max: inner.max.clone(),
                methods: methods?,
            })
        }
    }

    #[derive(Clone)]
    pub struct PayinMethod {
        pub kind: String,
        pub name: Option<String>,
        pub description: Option<String>,
        pub group: Option<String>,
        pub required_payment_details: Option<String>, // JSON serialized
        pub fee: Option<String>,
        pub min: Option<String>,
        pub max: Option<String>,
    }

    impl PayinMethod {
        pub fn to_inner(&self) -> Result<InnerPayinMethod> {
            let required_payment_details = match self.required_payment_details.clone() {
                None => None,
                Some(s) => Some(
                    serde_json::from_str::<serde_json::Value>(&s)
                        .map_err(|e| Arc::new(e.into()))?,
                ),
            };

            Ok(InnerPayinMethod {
                kind: self.kind.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                group: self.group.clone(),
                required_payment_details,
                fee: self.fee.clone(),
                min: self.min.clone(),
                max: self.max.clone(),
            })
        }

        pub fn from_inner(inner: InnerPayinMethod) -> Result<Self> {
            let required_payment_details = match inner.required_payment_details.clone() {
                None => None,
                Some(s) => Some(serde_json::to_string(&s).map_err(|e| Arc::new(e.into()))?),
            };

            Ok(Self {
                kind: inner.kind.clone(),
                name: inner.name.clone(),
                description: inner.description.clone(),
                group: inner.group.clone(),
                required_payment_details,
                fee: inner.fee.clone(),
                min: inner.min.clone(),
                max: inner.max.clone(),
            })
        }
    }

    #[derive(Clone)]
    pub struct PayoutDetails {
        pub currency_code: String,
        pub min: Option<String>,
        pub max: Option<String>,
        pub methods: Vec<PayoutMethod>,
    }

    impl PayoutDetails {
        pub fn to_inner(&self) -> Result<InnerPayoutDetails> {
            let methods: Result<Vec<InnerPayoutMethod>> = self
                .methods
                .clone()
                .into_iter()
                .map(|m| m.to_inner())
                .collect();

            Ok(InnerPayoutDetails {
                currency_code: self.currency_code.clone(),
                min: self.min.clone(),
                max: self.max.clone(),
                methods: methods?,
            })
        }

        pub fn from_inner(inner: InnerPayoutDetails) -> Result<Self> {
            let methods: Result<Vec<PayoutMethod>> = inner
                .methods
                .clone()
                .into_iter()
                .map(PayoutMethod::from_inner)
                .collect();

            Ok(Self {
                currency_code: inner.currency_code.clone(),
                min: inner.min.clone(),
                max: inner.max.clone(),
                methods: methods?,
            })
        }
    }

    #[derive(Clone)]
    pub struct PayoutMethod {
        pub kind: String,
        pub name: Option<String>,
        pub description: Option<String>,
        pub group: Option<String>,
        pub required_payment_details: Option<String>, // JSON serialized
        pub fee: Option<String>,
        pub min: Option<String>,
        pub max: Option<String>,
        pub estimated_settlement_time: i64,
    }

    impl PayoutMethod {
        pub fn to_inner(&self) -> Result<InnerPayoutMethod> {
            let required_payment_details = match self.required_payment_details.clone() {
                None => None,
                Some(s) => Some(
                    serde_json::from_str::<serde_json::Value>(&s)
                        .map_err(|e| Arc::new(e.into()))?,
                ),
            };

            Ok(InnerPayoutMethod {
                kind: self.kind.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                group: self.group.clone(),
                required_payment_details,
                fee: self.fee.clone(),
                min: self.min.clone(),
                max: self.max.clone(),
                estimated_settlement_time: self.estimated_settlement_time,
            })
        }

        pub fn from_inner(inner: InnerPayoutMethod) -> Result<Self> {
            let required_payment_details = match inner.required_payment_details.clone() {
                None => None,
                Some(s) => Some(serde_json::to_string(&s).map_err(|e| Arc::new(e.into()))?),
            };

            Ok(Self {
                kind: inner.kind.clone(),
                name: inner.name.clone(),
                description: inner.description.clone(),
                group: inner.group.clone(),
                required_payment_details,
                fee: inner.fee.clone(),
                min: inner.min.clone(),
                max: inner.max.clone(),
                estimated_settlement_time: inner.estimated_settlement_time,
            })
        }
    }
}
