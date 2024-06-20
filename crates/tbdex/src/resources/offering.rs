use super::{resource_metadata::ResourceMetadata, Resource, ResourceKind, Result};
use web5::apid::{
    credentials::presentation_definition::PresentationDefinition, dids::bearer_did::BearerDid,
};

#[derive(Clone)]
pub struct Offering {
    pub metadata: ResourceMetadata,
    pub data: OfferingData,
    pub signature: String,
}

impl Offering {
    pub fn new(from: String, data: OfferingData, protocol: String) -> Self {
        // 🚧 not functional
        Self {
            metadata: ResourceMetadata {
                kind: ResourceKind::Offering,
                from,
                to: String::default(),
                id: String::default(),
                protocol,
                created_at: String::default(),
                updated_at: None,
            },
            data,
            signature: String::default(),
        }
    }
}

impl Resource for Offering {
    fn sign(&self, _bearer_did: BearerDid) -> Result<()> {
        println!("Offering.sign() invoked");
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Offering.verify() invoked");
        Ok(())
    }
}

#[derive(Clone)]
pub struct OfferingData {
    pub description: String,
    pub payout_units_per_payin_unit: String,
    pub payin: PayinDetails,
    pub payout: PayoutDetails,
    pub required_claims: PresentationDefinition,
}

#[derive(Clone)]
pub struct PayinDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayinMethod>,
}

#[derive(Clone)]
pub struct PayinMethod {
    pub kind: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub required_payment_details: Option<String>, // 🚧 JsonNode
    pub fee: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Clone)]
pub struct PayoutDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayoutMethod>,
}

#[derive(Clone)]
pub struct PayoutMethod {
    pub kind: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub required_payment_details: Option<String>, // 🚧 JsonNode
    pub fee: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub estimated_settlement_time: i64,
}
