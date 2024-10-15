// import { Exchange, RfqData } from 'index';
import {
  Rfq,
  CreateRfqData,
  CreateSelectedPayinMethod,
  CreateSelectedPayoutMethod,
  Quote,
  Order,
  OrderInstructions,
  OrderStatus,
  Close,
  BearerDid,
  getOfferings,
  createExchange,
  getExchange,
  getExchangeIds,
  GetExchangeResponseBody,
  Exchange,
  RfqData,
  Message
} from 'tbdex';

import axios from "axios"

async function runHappyPathFlow(
  pfiDidUri: string,
  verifiableCredential: string,
  bearerDid: BearerDid
) {
  console.log('\n ~Running Happy Path Polling Flow~ \n');

  console.log('1. Fetching offerings...');
  const offerings = await getOfferings(pfiDidUri);
  // console.log(offerings)
  const offeringId = offerings[0].metadata.id;
  console.log(`Successfully fetched ${offeringId}\n`);

  console.log('2. Creating exchange...');
  const createRfqData : CreateRfqData = {
    claims: [verifiableCredential],
    offeringId: offeringId,
    payin: {
        amount: '101',
        kind: 'USD_LEDGER',
    },
    payout: {
        kind: 'MOMO_MPESA', 
    }
  };

  const rfq = Rfq.create(
    pfiDidUri,
    bearerDid.did.uri,
    createRfqData
  );

  await rfq.sign(bearerDid);
  await rfq.verify();

  await createExchange(rfq);

  console.log(`Created exchange ${rfq.metadata.exchangeId}\n`);

  const exchangeId = rfq.metadata.exchangeId;

  console.log('3. Waiting for Quote...');
  let quote: Quote;

  console.log("START TEST")

  // GetExchangeIdsQueryParams
  // const exchanges = await getExchangeIds(pfiDidUri, bearerDid, {})

  while (!quote) {
    await new Promise((resolve) => setTimeout(resolve, 500));
    console.log("Before get exchagne ")
    console.log(pfiDidUri)
    console.log(bearerDid)
    console.log(exchangeId)

    // const exchange = await getExchange(pfiDidUri, bearerDid, exchangeId);

    // TODO: Super hack while getExchange is not working...
    const response = await axios.get('http://localhost:8082/exchanges/' + exchangeId);
    console.log(response.data)
    const getExchangeResponseBody:GetExchangeResponseBody =  GetExchangeResponseBody.fromJSONString(JSON.stringify(response.data))
    console.log("EXCHANGE RESPONSE BODY:")
    console.log(JSON.stringify(getExchangeResponseBody.data))

    // exchangeResponseBody.
    console.log("EXCHANGE! Looking for quote...")
    // const exchange = getExchangeResponseBody[0].data as Exchange
    // const exchange = Exchange.fromJSONString(JSON.stringify(getExchangeResponseBody.data))
    console.log("First element of data:", JSON.stringify(getExchangeResponseBody.data[0], null, 2));

    // const exchange = Exchange.fromJSONString(JSON.stringify(getExchangeResponseBody.data[0]));

    const firstElement:Message = getExchangeResponseBody.data[0];

    const exchange = new Exchange(
      firstElement.data,  // Assuming this is the rfq part
      firstElement.quote,  // Or any other fields if necessary
      firstElement.order,
      firstElement.orderInstructions,
      firstElement.cancel,
      firstElement.orderStatuses,
      firstElement.close
    );
    //

    console.log(exchange)
    if (exchange.quote) {
      quote = exchange.quote;
    }
  }

  // console.log(`Received quote ${quote.metadata.id}\n`);

  // console.log('4. Submitting order...');
  // const order = Order.create(
  //   pfiDidUri,
  //   bearerDid.did.uri,
  //   quote.metadata.exchangeId
  // );

  // order.sign(bearerDid);
  // order.verify();

  // await tbdex.sdk.httpclient.submitOrder({ order });
  // console.log(`Order submitted ${order.metadata.id}\n`);

  // console.log('5. Waiting for order instructions...');
  // let orderInstructions: OrderInstructions | null = null;
  // while (!orderInstructions) {
  //   await new Promise((resolve) => setTimeout(resolve, 500));
  //   const exchange = await tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId);
  //   if (exchange.orderInstructions) {
  //     orderInstructions = exchange.orderInstructions;
  //     console.log(`Received order instructions: ${orderInstructions.metadata.id}\n`);
  //   }
  // }

  // console.log('6. Waiting for order status: PAYOUT_SETTLED...');
  // let orderStatuses: OrderStatus[] | null = null;
  // while (!orderStatuses) {
  //   await new Promise((resolve) => setTimeout(resolve, 500));
  //   const exchange = await tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId);
  //   if (exchange.orderStatuses) {
  //     for (const os of exchange.orderStatuses) {
  //       if (os.data.status.toString() === 'PAYOUT_SETTLED') {
  //         orderStatuses = exchange.orderStatuses;
  //         break;
  //       }
  //     }
  //   }
  // }

  // for (const orderStatus of orderStatuses!) {
  //   console.log(
  //     `Received order status ${orderStatus.metadata.id} ${orderStatus.data.status}\n`
  //   );
  // }

  // console.log('7. Waiting for Close...');
  // let close: Close | null = null;
  // while (!close) {
  //   await new Promise((resolve) => setTimeout(resolve, 500));
  //   const exchange = await tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId);
  //   if (exchange.close) {
  //     close = exchange.close;
  //   }
  // }

  // console.log(`Close received ${close.metadata.id} ${close.data.success}\n`);
  // console.log('Exchange completed successfully!');
}

// export default runHappyPathFlow;
export { runHappyPathFlow };

