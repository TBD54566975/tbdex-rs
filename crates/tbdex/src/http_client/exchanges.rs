use super::Result;
use crate::{
    http_client::generate_access_token,
    jose::Signer,
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
};
use reqwest::blocking::Client;
use serde::Serialize;
use web5::apid::{
    crypto::{
        jwk::Jwk,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager},
    },
    dids::bearer_did::BearerDid,
};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Exchange {
    pub rfq: Rfq,
    pub quote: Option<Quote>,
    pub order: Option<Order>,
    pub order_statuses: Option<Vec<OrderStatus>>,
    pub close: Option<Close>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
struct CreateExchangeRequest {
    rfq: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
}

pub fn create_exchange(rfq: Rfq, reply_to: Option<String>) -> Result<()> {
    // TODO did:dht resolution not functional
    let key_manager = InMemoryKeyManager::new();
    let public_jwk = key_manager
        .import_private_jwk(Jwk {
            crv: "Ed25519".to_string(),
            alg: "EdDSA".to_string(),
            kty: "OKP".to_string(),
            x: "kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw".to_string(),
            y: None,
            d: Some("jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04".to_string()),
        })
        .unwrap();
    let web5_signer = key_manager.get_signer(public_jwk).unwrap();
    let client_did_uri =
        "did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0".to_string();
    let jose_signer = Signer {
        kid: client_did_uri.clone(),
        web5_signer,
    };
    let endpoint = "http://localhost:9000/exchanges";
    // TODO all of the above

    let access_token = generate_access_token(&rfq.metadata.to, &client_did_uri, jose_signer)?;
    let create_exchange_request = CreateExchangeRequest { rfq, reply_to };
    let request_body = serde_json::to_string(&create_exchange_request)?;

    // todo handle error responses
    Client::new()
        .post(endpoint)
        .body(request_body)
        .bearer_auth(access_token)
        .send()?;

    Ok(())
}

pub fn submit_order(_order: Order) -> Result<()> {
    println!("TbdexHttpClient::submit_order() invoked");
    Ok(())
}

pub fn submit_close(_close: Close) -> Result<()> {
    println!("TbdexHttpClient::submit_close() invoked");
    Ok(())
}

pub fn get_exchange(
    _pfi_did: String,
    _requestor_did: BearerDid,
    _exchange_id: String,
) -> Result<Exchange> {
    println!("TbdexHttpClient::get_exchange() invoked");
    Ok(Exchange::default())
}

pub fn get_exchanges(_pfi_did: String, _requestor_did: BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::rfq::{CreateRfqData, CreateSelectedPayinMethod};
    use std::sync::Arc;

    #[test]
    fn can_create_exchange() {
        // TODO replace the below with did:dht once implemented
        let did_uri = "did:jwk:eyJrdHkiOiJPS1AiLCJ1c2UiOiJzaWciLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoiVnRTSFhQbEtEdzFFRW9PajVYTjNYV2hqU1BZVk52WC1lNHZqUk8weVlKQSIsIngiOiJpejcwc3ZTTHhOWmhzRHhlSlFfam5PVmJYM0tGTmtjQmNNaldqWm1YRXNBIiwiYWxnIjoiRWREU0EifQ";
        let key_manager = InMemoryKeyManager::new();
        let bearer_did = BearerDid::new(did_uri, Arc::new(key_manager)).unwrap();
        // TODO the above

        let pfi_did = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy".to_string();

        // TODO we really need bearer DIDs at this point
        // let rfq = Rfq::new(
        //     bearer_did,
        //     "did:test:pfi".to_string(),
        //     did_uri.to_string(),
        //     CreateRfqData {
        //         offering_id: "offering_123".to_string(),
        //         payin: CreateSelectedPayinMethod {
        //             kind: "BTC".to_string(),
        //             payment_details: serde_json::json!({"tmp": "payment-details"}),
        //             amount: "101".to_string(),
        //         },
        //         payout: CreateSelectedPayoutMethod {
        //             kind: "BTC".to_string(),
        //             payment_details: serde_json::json!({"tmp": "payment-details"}),
        //         },
        //         claims: vec!["some-claim".to_string()],
        //     },
        //     "1.0".to_string(),
        //     None,
        // )
        // .unwrap();

        // let balances = create_exchange(pfi_did, bearer_did).unwrap();
        // assert_ne!(0, balances.len());
    }
}
