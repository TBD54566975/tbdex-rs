use crate::{errors::Result, get_rt, resources::balance::Balance};
use std::sync::{Arc, RwLock};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub fn get_balances(pfi_did_uri: String, bearer_did: Arc<BearerDid>) -> Result<Vec<Arc<Balance>>> {
    let rt = get_rt()?;
    let inner_balances = rt.block_on(tbdex::http_client::balances::get_balances(
        &pfi_did_uri,
        &bearer_did.0.clone(),
    ))?;

    let balances = inner_balances
        .into_iter()
        .map(|b| Arc::new(Balance(Arc::new(RwLock::new(b)))))
        .collect();

    Ok(balances)
}
