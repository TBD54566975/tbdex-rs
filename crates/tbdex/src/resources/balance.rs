use super::{resource_metadata::ResourceMetadata, Resource, ResourceKind, Result};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct Balance {
    pub metadata: ResourceMetadata,
    pub data: BalanceData,
    pub signature: String,
}

impl Balance {
    pub fn new(from: String, data: BalanceData, protocol: String) -> Self {
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

impl Resource for Balance {
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
pub struct BalanceData {
    pub currency_code: String,
    pub available: String,
}
