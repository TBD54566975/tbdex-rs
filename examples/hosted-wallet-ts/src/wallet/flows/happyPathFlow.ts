import axios from 'axios';
import { Message, Balance, Rfq, CreateExchangeRequestBody } from "tbdex";
import { BalanceData, CreateSelectedPayinMethod, PayinMethod } from "tbdex/dist/wasm/generated-mappings";

export const runHappyPathFlow = async (
  pfiDidUri: string,
  verifiableCredential: string,
  replyToUrl: string
) => {
  console.log('\n ~Running Happy Path Webhook Flow~ \n');

  const balanceData: BalanceData = {available: "av",currencyCode: "cc"};
  console.log(balanceData);
  console.log("test finished");

  //   // TODO: Implement the flow logic
  //   console.log('1. Fetching offerings...');
  //   try {
  //     const response = await axios.get(`${pfiDidUri}/offerings`);
  //     const offerings = response.data.offerings;
  //     console.log('Successfully fetched offerings:', offerings);

  //     // Continue with the flow...
  //   } catch (error) {
  //     console.error('Error fetching offerings:', error);
  //   }
  // };
};


