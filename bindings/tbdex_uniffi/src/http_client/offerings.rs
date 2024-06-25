use crate::{errors::Result, resources::offering::Offering};
use std::sync::{Arc, RwLock};

pub fn get_offerings(pfi_did_uri: String) -> Result<Vec<Arc<Offering>>> {
    let inner_offerings = tbdex::http_client::offerings::get_offerings(&pfi_did_uri)
        .map_err(|e| Arc::new(e.into()))?;

    let offerings = inner_offerings
        .into_iter()
        .map(|o| Arc::new(Offering(Arc::new(RwLock::new(o)))))
        .collect();

    Ok(offerings)
}
