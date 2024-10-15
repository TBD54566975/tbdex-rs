// import 'dotenv/config'
import pfiPortableDidJson from './pfiportabledid.json' assert { type: 'json' };


import { GetExchangeIdsQueryParams, PortableDid, BearerDid } from 'tbdex';
import { runHappyPathFlow } from "./happypathpollingflow.js"
// import { BearerDid, PortableDid } from 'web5-sdk/dids';
// import * as fs from 'fs';
// import * as path from 'path';

// const portableDID: PortableDid = JSON.parse(
//   '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
// );

const hostedWalletPortableDid: PortableDid = JSON.parse(
  '{"uri":"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy","document":{"id":"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy","verificationMethod":[{"id":"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0","type":"JsonWebKey","controller":"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy","publicKeyJwk":{"crv":"Ed25519","kty":"OKP","x":"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw","kid":"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI","alg":"EdDSA"}}],"authentication":["did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0"],"assertionMethod":["did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0"],"capabilityDelegation":["did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0"],"capabilityInvocation":["did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0"]},"privateKeys":[{"crv":"Ed25519","d":"jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04","kty":"OKP","x":"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw","kid":"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI","alg":"EdDSA"}]}'
)

const walletBearerDid = BearerDid.fromPortableDID(hostedWalletPortableDid);


const pfiDidUri = "did:dht:ysyokwn6mxnzihgnhkkesjig8cdb3r94eq8abp3a7e935y4s3c4y"

enum ExchangeFlowType {
  HAPPY_PATH_POLLING_FLOW,
  HAPPY_PATH_WEBHOOK_FLOW,
  CANCEL_FLOW,
  ERROR_FLOW,
  ALL_FLOWS,
}

// Set the desired flow type here
// const FLOW_TYPE = ExchangeFlowType.HAPPY_PATH_POLLING_FLOW;

const FLOW_TYPE: ExchangeFlowType = ExchangeFlowType.HAPPY_PATH_POLLING_FLOW;


// console.log(process.env)

async function main() {
  // readEnv();

  // console.log(process.env.PFI_PORTABLE_DID)

  //TOOD: Use this
  // const pfiPortableDid: PortableDid = JSON.parse(process.env.PFI_PORTABLE_DID)


  const pfiPortableDid: PortableDid = pfiPortableDidJson;
  const pfiBearerDid: BearerDid = BearerDid.fromPortableDID(pfiPortableDid);

  const vc = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJTYW5jdGlvbkNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDp5c3lva3duNm14bnppaGduaGtrZXNqaWc4Y2RiM3I5NGVxOGFicDNhN2U5MzV5NHMzYzR5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNy0wMlQwNDoyNDoxNC4yNzYzMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OjFmczVobnhzZ3R4Z2RyNHd6cWkzOGNuajQ2YjF3aGhuOTRvandvNjZnOGhzYzVidDNmZ3kifX0sImlzcyI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSIsImp0aSI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsIm5iZiI6MTcxOTg5NDI1NCwiaWF0IjoxNzE5ODk0MjU0fQ.c4ws9jR28jElo_uaW9l5OTL-IPMx4JxWl4De7l_BTk0qNhcFlRtR-U0b9087CUOdpNu25XGZzn-R_EVImRGgCw"

  // const pfiDidUri = pfiBearerDid.did.uri;

  // console.log("HRE")
  // console.log(pfiDidUri)
  // const verifiableCredential = process.env.HOSTED_WALLET_VERIFIABLE_CREDENTIAL!;
  // const bearerDid = BearerDid.fromPortableDid(
  //   PortableDid.fromJsonString(process.env.HOSTED_WALLET_PORTABLE_DID_JSON!)
  // );
  // const replyToUrl = process.env.REPLY_TO_URL!;

  switch (FLOW_TYPE) {
    case ExchangeFlowType.HAPPY_PATH_POLLING_FLOW:
      await runHappyPathFlow(pfiDidUri, vc, walletBearerDid);
      break;
    // case ExchangeFlowType.HAPPY_PATH_WEBHOOK_FLOW:
    //   await runHappyPathPollingFlow(pfiDidUri, verifiableCredential, bearerDid);
    //   break;
    // case ExchangeFlowType.CANCEL_FLOW:
    //   await runCancelFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl);
    //   break;
    // case ExchangeFlowType.ERROR_FLOW:
    //   await runErrorFlow(pfiDidUri, verifiableCredential, bearerDid);
    //   break;
    // case ExchangeFlowType.ALL_FLOWS:
    //   await runHappyPathFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl);
    //   await runHappyPathPollingFlow(pfiDidUri, verifiableCredential, bearerDid);
    //   await runCancelFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl);
    //   await runErrorFlow(pfiDidUri, verifiableCredential, bearerDid);

      // const allExchanges = await tbdex.sdk.httpclient.getExchangeIds(pfiDidUri, bearerDid);
      // console.log(`All Exchanges Completed: ${allExchanges}`);

      // const paginatedExchanges = await tbdex.sdk.httpclient.getExchangeIds(
      //   pfiDidUri,
      //   bearerDid,
      //   new GetExchangeIdsQueryParams(1, 2)
      // );
      // console.log(`Paginated Exchanges: ${paginatedExchanges}`);
      // break;
  }
}

main().catch((error) => {

  console.error('Error in flow! Exiting! Error:\n', error);
});