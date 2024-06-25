use crate::{
    errors::{Result, RustCoreError},
    resources::offering::Offering,
};
use std::sync::{Arc, RwLock};
use tbdex::messages::rfq::Rfq as InnerRfq;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Rfq(pub Arc<RwLock<InnerRfq>>);

impl Rfq {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        create_rfq_data: data::CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let rfq = InnerRfq::new(
            &bearer_did.0.clone(),
            &to,
            &from,
            &create_rfq_data.to_inner()?,
            &protocol,
            external_id,
        )
        .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(rfq))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_rfq = InnerRfq::from_json_string(json).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_rfq))))
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        inner_rfq.to_json().map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> Result<data::Rfq> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(data::Rfq {
            metadata: rfq.metadata.clone(),
            data: data::RfqData::from_inner(rfq.data.clone())?,
            private_data: data::RfqPrivateData::from_inner(rfq.private_data.clone())?,
            signature: rfq.signature.clone(),
        })
    }

    pub fn to_inner(&self) -> Result<InnerRfq> {
        let inner_rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(inner_rfq.clone())
    }

    pub fn verify_offering_requirements(&self, offering: Arc<Offering>) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_offering_requirements(offering.to_inner()?)
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_all_private_data(&self) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_all_private_data()
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_present_private_data(&self) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_present_private_data()
            .map_err(|e| Arc::new(e.into()))
    }
}

pub mod data {
    use super::*;
    use tbdex::messages::{
        rfq::{
            CreateRfqData as InnerCreateRfqData,
            CreateSelectedPayinMethod as InnerCreateSelectedPayinMethod,
            CreateSelectedPayoutMethod as InnerCreateSelectedPayoutMethod,
            PrivatePaymentDetails as InnerPrivatePaymentDetails, RfqData as InnerRfqData,
            RfqPrivateData as InnerRfqPrivateData, SelectedPayinMethod as InnerSelectedPayinMethod,
            SelectedPayoutMethod as InnerSelectedPayoutMethod,
        },
        MessageMetadata,
    };

    #[derive(Clone)]
    pub struct CreateRfqData {
        pub offering_id: String,
        pub payin: CreateSelectedPayinMethod,
        pub payout: CreateSelectedPayoutMethod,
        pub claims: Vec<String>,
    }

    impl CreateRfqData {
        pub fn to_inner(&self) -> Result<InnerCreateRfqData> {
            Ok(InnerCreateRfqData {
                offering_id: self.offering_id.clone(),
                payin: self.payin.to_inner()?,
                payout: self.payout.to_inner()?,
                claims: self.claims.clone(),
            })
        }
    }

    #[derive(Clone)]
    pub struct CreateSelectedPayinMethod {
        pub kind: String,
        pub payment_details: Option<String>, // JSON serialized
        pub amount: String,
    }

    impl CreateSelectedPayinMethod {
        pub fn to_inner(&self) -> Result<InnerCreateSelectedPayinMethod> {
            let payment_details = match &self.payment_details {
                Some(pd) => Some(
                    serde_json::from_str::<serde_json::Value>(pd)
                        .map_err(|e| Arc::new(e.into()))?,
                ),
                None => None,
            };
            Ok(InnerCreateSelectedPayinMethod {
                kind: self.kind.clone(),
                payment_details,
                amount: self.amount.clone(),
            })
        }
    }

    #[derive(Clone)]
    pub struct CreateSelectedPayoutMethod {
        pub kind: String,
        pub payment_details: Option<String>, // JSON serialized
    }

    impl CreateSelectedPayoutMethod {
        pub fn to_inner(&self) -> Result<InnerCreateSelectedPayoutMethod> {
            let payment_details = match &self.payment_details {
                Some(pd) => Some(
                    serde_json::from_str::<serde_json::Value>(pd)
                        .map_err(|e| Arc::new(e.into()))?,
                ),
                None => None,
            };
            Ok(InnerCreateSelectedPayoutMethod {
                kind: self.kind.clone(),
                payment_details,
            })
        }
    }

    pub struct Rfq {
        pub metadata: MessageMetadata,
        pub data: RfqData,
        pub private_data: RfqPrivateData,
        pub signature: String,
    }

    #[derive(Clone)]
    pub struct RfqData {
        pub offering_id: String,
        pub payin: SelectedPayinMethod,
        pub payout: SelectedPayoutMethod,
        pub claims_hash: Option<String>,
    }

    impl RfqData {
        pub fn from_inner(inner: InnerRfqData) -> Result<Self> {
            Ok(Self {
                offering_id: inner.offering_id,
                payin: SelectedPayinMethod::from_inner(inner.payin)?,
                payout: SelectedPayoutMethod::from_inner(inner.payout)?,
                claims_hash: inner.claims_hash,
            })
        }
    }

    #[derive(Clone)]
    pub struct SelectedPayinMethod {
        pub kind: String,
        pub payment_details_hash: Option<String>,
        pub amount: String,
    }

    impl SelectedPayinMethod {
        pub fn from_inner(inner: InnerSelectedPayinMethod) -> Result<Self> {
            Ok(Self {
                kind: inner.kind,
                payment_details_hash: inner.payment_details_hash,
                amount: inner.amount,
            })
        }
    }

    #[derive(Clone)]
    pub struct SelectedPayoutMethod {
        pub kind: String,
        pub payment_details_hash: Option<String>,
    }

    impl SelectedPayoutMethod {
        pub fn from_inner(inner: InnerSelectedPayoutMethod) -> Result<Self> {
            Ok(Self {
                kind: inner.kind,
                payment_details_hash: inner.payment_details_hash,
            })
        }
    }

    #[derive(Clone)]
    pub struct RfqPrivateData {
        pub salt: String,
        pub payin: Option<PrivatePaymentDetails>,
        pub payout: Option<PrivatePaymentDetails>,
        pub claims: Option<Vec<String>>,
    }

    impl RfqPrivateData {
        pub fn from_inner(inner: InnerRfqPrivateData) -> Result<Self> {
            Ok(Self {
                salt: inner.salt,
                payin: inner
                    .payin
                    .map(PrivatePaymentDetails::from_inner)
                    .transpose()?,
                payout: inner
                    .payout
                    .map(PrivatePaymentDetails::from_inner)
                    .transpose()?,
                claims: inner.claims,
            })
        }
    }

    #[derive(Clone)]
    pub struct PrivatePaymentDetails {
        pub payment_details: Option<String>, // JSON serialized
    }

    impl PrivatePaymentDetails {
        pub fn from_inner(inner: InnerPrivatePaymentDetails) -> Result<Self> {
            let payment_details = match &inner.payment_details {
                Some(pd) => Some(serde_json::to_string(pd).map_err(|e| Arc::new(e.into()))?),
                None => None,
            };
            Ok(Self { payment_details })
        }
    }
}
