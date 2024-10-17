import {
  Rfq,
  CreateRfqData,
  Quote,
  Order,
  OrderInstructions,
  OrderStatus,
  Close,
  BearerDid,
  getOfferings,
  createExchange,
  submitOrder,
  GetExchangeResponseBody,
  Message,
  getExchange,
} from 'tbdex';
import axios from 'axios';

async function runHappyPathFlow(
  pfiDidUri: string,
  verifiableCredential: string,
  bearerDid: BearerDid
) {
  console.log('\n~ Running Happy Path Polling Flow ~\n');

  try {
    // Step 1: Fetch offerings
    console.log('1. Fetching offerings...');
    const offerings = await getOfferings(pfiDidUri);
    if (!offerings || offerings.length === 0) {
      throw new Error('No offerings available.');
    }
    const offeringId = offerings[0].metadata.id;
    console.log(`Successfully fetched offering ID: ${offeringId}\n`);

    // Step 2: Create exchange (RFQ)
    console.log('2. Creating exchange...');
    const createRfqData: CreateRfqData = {
      claims: [verifiableCredential],
      offeringId,
      payin: {
        amount: '101',
        kind: 'USD_LEDGER',
      },
      payout: {
        kind: 'MOMO_MPESA',
      },
    };

    const rfq = Rfq.create(pfiDidUri, bearerDid.did.uri, createRfqData);
    await rfq.sign(bearerDid);
    await rfq.verify();
    await createExchange(rfq);

    const exchangeId = rfq.metadata.exchangeId;
    console.log(`Created exchange with ID: ${exchangeId}\n`);

    // Step 3: Wait for Quote
    console.log('3. Waiting for Quote...');
    let quote: Quote | undefined;
    while (!quote) {
      await delay(500);
      const exchangeData = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      for (const element of exchangeData) {
        if (element.metadata.kind === 'quote') {
          console.log('Found quote!');
          quote = Quote.fromJSONString(JSON.stringify(element));
          break;
        }
      }
    }
    console.log(`Received quote with ID: ${quote.metadata.id}\n`);

    // Step 4: Submit Order
    console.log('4. Submitting order...');
    const order = Order.create(pfiDidUri, bearerDid.did.uri, exchangeId);
    await order.sign(bearerDid);
    await order.verify();
    await submitOrder(order);
    console.log(`Order submitted with ID: ${order.metadata.id}\n`);

    // Step 5: Wait for Order Instructions
    console.log('5. Waiting for Order Instructions...');
    let orderInstructions: OrderInstructions | undefined;
    while (!orderInstructions) {
      await delay(500);
      const exchangeData = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      for (const element of exchangeData) {
        if (element.metadata.kind === 'orderinstructions') {
          console.log('Found order instructions!');
          orderInstructions = OrderInstructions.fromJSONString(
            JSON.stringify(element)
          );
          break;
        }
      }
    }
    console.log(
      `Received order instructions with ID: ${orderInstructions.metadata.id}\n`
    );

    // Step 6: Wait for Order Status: PAYOUT_SETTLED
    console.log('6. Waiting for Order Status: PAYOUT_SETTLED...');
    let payoutSettled = false;
    while (!payoutSettled) {
      await delay(500);
      const exchangeData = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      for (const element of exchangeData) {
        if (element.metadata.kind === 'orderstatus') {
          const orderStatus = OrderStatus.fromJSONString(JSON.stringify(element));
          if (orderStatus.data.status === 'PAYOUT_SETTLED') {
            console.log('Order status PAYOUT_SETTLED received.');
            payoutSettled = true;
            break;
          }
        }
      }
    }
    console.log('Order status PAYOUT_SETTLED confirmed.\n');

    // Step 7: Wait for Close
    console.log('7. Waiting for Close...');
    let close: Close | undefined;
    while (!close) {
      await delay(500);
      const exchangeData = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      for (const element of exchangeData) {
        if (element.metadata.kind === 'close') {
          console.log('Found close message!');
          close = Close.fromJSONString(JSON.stringify(element));
          break;
        }
      }
    }

    console.log(
      `Exchange closed with ID: ${close.metadata.id}, Success: ${close.data.success}\n`
    );
    console.log('Exchange completed successfully!');
  } catch (error) {
    console.error('Error during Happy Path Flow:', error);
  }
}

/**
 * Helper function to fetch exchange data
 * @param pfiDidUri - The PFI DID URI
 * @param bearerDid - The Bearer DID
 * @param exchangeId - The exchange ID
 * @returns An array of messages from the exchange
 */
async function fetchExchangeData(
  pfiDidUri: string,
  bearerDid: BearerDid,
  exchangeId: string
): Promise<Message[]> {
  try {
    // TODO: This is not working and returning an unknwon WASM exception, to get around this I'm using axios instead of our library for getExchange
    // const exchange = await getExchange(pfiDidUri, bearerDid, exchangeId);
    // console.log(JSON.stringify(exchange));

    const response = await axios.get('http://localhost:8082/exchanges/' + exchangeId);

    const getExchangeResponseBody = GetExchangeResponseBody.fromJSONString(
      JSON.stringify(response.data)
    );
    return getExchangeResponseBody.data;
  } catch (error) {
    console.error('Error fetching exchange data:', error);
    return [];
  }
}

/**
 * Helper function to introduce a delay
 * @param ms - Milliseconds to wait
 * @returns A promise that resolves after the specified delay
 */
function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export { runHappyPathFlow };