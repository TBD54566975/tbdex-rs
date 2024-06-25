use std::{sync::Arc, time::Duration};
use tbdex::{
    http_client::{
        exchanges::{create_exchange, get_exchange, submit_order, Exchange},
        offerings::get_offerings,
    },
    messages::{
        order::Order,
        rfq::{CreateRfqData, CreateSelectedPayinMethod, CreateSelectedPayoutMethod, Rfq},
    },
};
use web5::apid::{
    crypto::{jwk::Jwk, key_managers::in_memory_key_manager::InMemoryKeyManager},
    dids::{
        bearer_did::BearerDid,
        data_model::{document::Document, verification_method::VerificationMethod},
        did::Did,
    },
};

fn main() {
    let pfi_did_uri = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy".to_string();

    let did_uri = "did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy".to_string();
    let key_manager = InMemoryKeyManager::new();
    key_manager
        .import_private_jwk(Jwk {
            crv: "Ed25519".to_string(),
            alg: "EdDSA".to_string(),
            kty: "OKP".to_string(),
            x: "kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw".to_string(),
            y: None,
            d: Some("jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04".to_string()),
        })
        .unwrap();
    let bearer_did = BearerDid {
        did: Did::new(&did_uri).unwrap(),
        document: Document {
            id: did_uri.clone(),
            verification_method: vec![VerificationMethod {
                id: format!("{}#0", did_uri),
                r#type: "JsonWebKey".to_string(),
                controller: did_uri.clone(),
                public_key_jwk: Jwk {
                    crv: "Ed25519".to_string(),
                    kty: "OKP".to_string(),
                    alg: "EdDSA".to_string(),
                    x: "kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw".to_string(),
                    ..Default::default()
                },
            }],
            ..Default::default()
        },
        key_manager: Arc::new(key_manager),
    };

    // request offerings
    let offerings = get_offerings(&pfi_did_uri).unwrap();
    assert_ne!(0, offerings.len());

    // TODO pfi exemplar balances are missing `signature`
    // // request balance
    // let balances = get_balances(&pfi_did_uri, &bearer_did).unwrap();
    // assert_ne!(0, balances.len());

    // create exchange
    let rfq = Rfq::new(
            &bearer_did,
            &pfi_did_uri,
            &bearer_did.did.uri,
            &CreateRfqData {
                offering_id: offerings[0].metadata.id.clone(),
                payin: CreateSelectedPayinMethod {
                    kind: "USD_LEDGER".to_string(),
                    payment_details: None,
                    amount: "101".to_string(),
                },
                payout: CreateSelectedPayoutMethod {
                    kind: "MOMO_MPESA".to_string(),
                    payment_details: Some(serde_json::json!({
                        "phoneNumber": "867-5309",
                        "reason": "cause"
                    })),
                },
                claims: vec!["eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiU2FuY3Rpb25DcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6ZjBkYWNlZmItNDVlNy00YWEyLTkxNDctMTZmYTBiYzc3ZTVjIiwiaXNzdWVyIjoiZGlkOmRodDpjOGQ5aHVrN25ob200bjd6Z3JtYTZwanljeTY3NHVyYWE0cG9wOXV1dDQxZ2I5d3k4c255IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNi0yNFQxNDoxNTozNVoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImJlZXAiOiJib29wIn19LCJuYmYiOjE3MTkyMzg1MzUsImp0aSI6InVybjp1dWlkOmYwZGFjZWZiLTQ1ZTctNGFhMi05MTQ3LTE2ZmEwYmM3N2U1YyIsImlzcyI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImlhdCI6MTcxOTIzODUzNX0.DvDFIl8BTuHRk7VkB82OhYpX0WzBb3BucvAqfXiS92QCiRokXCgQAsOwbbSODoDaFWbHG0BJmWM-eDPcCoucCw".to_string()],
            },
            "1.0",
            None,
        )
        .unwrap();

    create_exchange(&rfq, None).unwrap();

    // get quote
    let exchange = get_exchange(&pfi_did_uri, &bearer_did, &rfq.metadata.exchange_id).unwrap();
    let quote = exchange.quote.unwrap();

    // submit order
    submit_order(
        &Order::new(
            &bearer_did,
            &pfi_did_uri,
            &bearer_did.did.uri,
            &quote.metadata.exchange_id,
            "1.0",
            None,
        )
        .unwrap(),
    )
    .unwrap();

    // get order status and close
    let mut exchange = Exchange::default();
    let mut count = 0;
    while exchange.close.is_none() {
        std::thread::sleep(Duration::from_secs(5));
        exchange = get_exchange(&pfi_did_uri, &bearer_did, &rfq.metadata.exchange_id).unwrap();

        count += 1;
        if count >= 3 {
            panic!("Failed to close exchange after 3 attempts");
        }
    }

    println!("Exchange completed successfully!")
}
