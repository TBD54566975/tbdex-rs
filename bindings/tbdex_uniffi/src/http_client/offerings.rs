use crate::{errors::Result, get_rt, resources::offering::Offering};
use std::sync::{Arc, RwLock};

pub fn get_offerings(pfi_did_uri: String) -> Result<Vec<Arc<Offering>>> {
    let rt = get_rt()?;
    let inner_offerings =
        rt.block_on(tbdex::http_client::offerings::get_offerings(&pfi_did_uri))?;

    let offerings = inner_offerings
        .into_iter()
        .map(|o| Arc::new(Offering(Arc::new(RwLock::new(o)))))
        .collect();

    Ok(offerings)
}
