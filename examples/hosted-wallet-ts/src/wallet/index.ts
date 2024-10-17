import dotenv from 'dotenv';
dotenv.config();

import { BearerDid } from 'tbdex';
import { runHappyPathFlow } from './happypathpollingflow.js';
import { PortableDid } from 'index';

const pfiPortableDid: PortableDid = JSON.parse(process.env.PFI_PORTABLE_DID);
const hostedWalletPortableDid: PortableDid = JSON.parse(process.env.HOSTED_WALLET_PORTABLE_DID_JSON);
const verifiableCredential = process.env.HOSTED_WALLET_VERIFIABLE_CREDENTIAL;

const pfiBearerDid = BearerDid.fromPortableDID(pfiPortableDid);
const walletBearerDid = BearerDid.fromPortableDID(hostedWalletPortableDid);

const pfiDidUri = pfiBearerDid.did.uri;

// Enum for exchange flow types
enum ExchangeFlowType {
  HAPPY_PATH_POLLING_FLOW,
  HAPPY_PATH_WEBHOOK_FLOW,
  CANCEL_FLOW,
  ERROR_FLOW,
  ALL_FLOWS,
}

// Set the desired flow type here
const FLOW_TYPE: ExchangeFlowType = ExchangeFlowType.HAPPY_PATH_POLLING_FLOW;

async function main() {
  try {
    switch (FLOW_TYPE) {
      case ExchangeFlowType.HAPPY_PATH_POLLING_FLOW:
        await runHappyPathFlow(pfiDidUri, verifiableCredential, walletBearerDid);
        break;
      // Additional cases can be added here for other flow types
      default:
        console.error('Unsupported flow type.');
        break;
    }
  } catch (error) {
    console.error('Error in flow! Exiting! Error:\n', error);
    process.exit(1);
  }
}

main();