import pfiPortableDidJson from './pfiportabledid.json' assert { type: 'json' };
import hostedWalletPortableDidJson from './hostedwalletportabledid.json' assert { type: 'json' };
import { BearerDid } from 'tbdex';
import { runHappyPathFlow } from './happypathpollingflow.js';

const pfiPortableDid = pfiPortableDidJson;
const hostedWalletPortableDid = hostedWalletPortableDidJson;

const pfiBearerDid = BearerDid.fromPortableDID(pfiPortableDid);
const walletBearerDid = BearerDid.fromPortableDID(hostedWalletPortableDid);

const pfiDidUri = pfiBearerDid.did.uri;

const verifiableCredential = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJTYW5jdGlvbkNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDp5c3lva3duNm14bnppaGduaGtrZXNqaWc4Y2RiM3I5NGVxOGFicDNhN2U5MzV5NHMzYzR5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNy0wMlQwNDoyNDoxNC4yNzYzMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OjFmczVobnhzZ3R4Z2RyNHd6cWkzOGNuajQ2YjF3aGhuOTRvandvNjZnOGhzYzVidDNmZ3kifX0sImlzcyI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSIsImp0aSI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsIm5iZiI6MTcxOTg5NDI1NCwiaWF0IjoxNzE5ODk0MjU0fQ.c4ws9jR28jElo_uaW9l5OTL-IPMx4JxWl4De7l_BTk0qNhcFlRtR-U0b9087CUOdpNu25XGZzn-R_EVImRGgCw';

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