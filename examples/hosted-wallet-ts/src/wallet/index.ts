import dotenv from 'dotenv';
import { runHappyPathFlow } from './flows/happyPathFlow.js';

dotenv.config();

enum ExchangeFlowType {
  HAPPY_PATH_WEBHOOK_FLOW,
  CANCEL_FLOW,
  ERROR_FLOW,
  ALL_FLOWS
}

let FLOW_TYPE = ExchangeFlowType.HAPPY_PATH_WEBHOOK_FLOW;

const pfiDidUri = process.env.PFI_DID_URI || '';
const verifiableCredential = process.env.HOSTED_WALLET_VERIFIABLE_CREDENTIAL || '';
const replyToUrl = process.env.REPLY_TO_URL || '';

switch (FLOW_TYPE) {
  case ExchangeFlowType.HAPPY_PATH_WEBHOOK_FLOW:
    runHappyPathFlow(pfiDidUri, verifiableCredential, replyToUrl);
    break;
  // case ExchangeFlowType.CANCEL_FLOW:
  //   // runCancelFlow(pfiDidUri, verifiableCredential, replyToUrl);
  //   break;
  // case ExchangeFlowType.ERROR_FLOW:
  //   // runErrorFlow(pfiDidUri, verifiableCredential);
  //   break;
  // case ExchangeFlowType.ALL_FLOWS:
  //   // runHappyPathFlow(pfiDidUri, verifiableCredential, replyToUrl);
  //   // runCancelFlow(pfiDidUri, verifiableCredential, replyToUrl);
  //   // runErrorFlow(pfiDidUri, verifiableCredential);
  //   break;
  default:
    console.log('Invalid flow type selected.');
}
