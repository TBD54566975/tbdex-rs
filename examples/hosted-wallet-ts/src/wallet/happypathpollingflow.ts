import {
  Rfq,
  CreateRfqData,
  Quote,
  Order,
  OrderInstructions,
  Close,
  BearerDid,
  getOfferings,
  createExchange,
  submitOrder,
  getExchange,
  Exchange,
} from 'tbdex';

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
        paymentDetails: null,
      },
      payout: {
        kind: 'MOMO_MPESA',
        paymentDetails: {
          phoneNumber: '867-5309',
          reason: 'cause',
        },
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
      const exchange = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      quote = exchange.quote;
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
      const exchange = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      orderInstructions = exchange.orderInstructions;
    }
    console.log(`Received order instructions with ID: ${orderInstructions.metadata.id}\n`);

    // Step 6: Wait for Order Status: PAYOUT_SETTLED
    console.log('6. Waiting for Order Status: PAYOUT_SETTLED...');
    let payoutSettled = false;
    while (!payoutSettled) {
      await delay(500);
      const exchange = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      const orderStatuses = exchange.orderStatuses;

      for(const orderStatus of orderStatuses) {
        if (orderStatus.data.status === 'PAYOUT_SETTLED') {
          console.log('Order status PAYOUT_SETTLED received.');
          payoutSettled = true;
          break;
        }
      }
    }
    console.log('Order status PAYOUT_SETTLED confirmed.\n');

    // Step 7: Wait for Close
    console.log('7. Waiting for Close...');
    let close: Close | undefined;
    while (!close) {
      await delay(500);
      const exchange = await fetchExchangeData(pfiDidUri, bearerDid, exchangeId);
      close = exchange.close;
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
): Promise<Exchange> {
  try {
    const exchange = await getExchange(pfiDidUri, bearerDid, exchangeId);
    return exchange;
  } catch (error) {
    console.error('Error fetching exchange data:', error);
    return;
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