import { Router } from 'express';
import axios from 'axios';
import {
  BearerDid,
  Offering,
  GetExchangeResponseBody,
  CreateExchangeRequestBody,
  UpdateExchangeRequestBody,
  ReplyToRequestBody,
  Quote,
  OrderInstructions,
  OrderStatus,
  Close,
  Rfq,
  Order,
  Cancel,
  Message,
  GetExchangesResponseBody,
  QuoteData,
  OrderInstructionsData,
  OrderStatusData,
  CloseData,
  ORDER_STATUS_KIND,
} from 'tbdex';
import { OfferingRepository } from '../data/offering-repository';

interface Exchange {
  rfq?: Message;
  quote?: Message;
  order?: Message;
  orderInstructions?: Message;
  cancel?: Message;
  close?: Message;
  orderStatuses?: Message[];
}

export default function exchangesRouter(
  bearerDid: BearerDid,
  offeringsRepository: OfferingRepository
) {
  const router = Router();

  // In-memory storage for exchanges and reply URLs
  const exchangesToReplyTo = new Map<string, string>();
  const exchangeIdToExchange = new Map<string, Exchange>();

  // Helper function to introduce delays in async functions
  const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

  /**
   * GET /exchanges
   * Retrieves a list of exchange IDs with pagination support
   */
  router.get('/', (req, res) => {
    console.log('GET /exchanges');

    try {
      const offset = parseInt(req.query['page[offset]'] as string) || 0;
      const limit = parseInt(req.query['page[limit]'] as string) || 10;

      const exchangeIds = Array.from(exchangeIdToExchange.keys());
      const paginatedExchanges = exchangeIds.slice(offset, offset + limit);

      const responseBody = new GetExchangesResponseBody(paginatedExchanges);

      res.type('application/json');
      res.send(responseBody.toJSONString());
    } catch (error) {
      console.error('Error in GET /exchanges:', error);
      res.status(500).json({ error: 'Internal server error' });
    }
  });

  /**
   * GET /exchanges/:id
   * Retrieves messages associated with a specific exchange ID
   */
  router.get('/:id', (req, res) => {
    const exchangeId = req.params.id;
    console.log(`GET /exchanges/${exchangeId}`);

    const exchange = exchangeIdToExchange.get(exchangeId);
    if (!exchange) {
      res.status(404).json({ error: 'Exchange not found' });
      return;
    }

    const messages = [
      exchange.rfq,
      exchange.quote,
      exchange.order,
      exchange.orderInstructions,
      exchange.cancel,
      exchange.close,
      ...(exchange.orderStatuses || []),
    ].filter(Boolean);

    const responseBody = new GetExchangeResponseBody(messages as Message[]);

    res.type('application/json');
    res.send(responseBody.toJSONString());
  });

  /**
   * POST /exchanges
   * Handles creation of a new exchange (RFQ)
   */
  router.post('/', async (req, res) => {
    console.log('POST /exchanges');

    try {
      const createExchangeRequestBody = CreateExchangeRequestBody.fromJSONString(
        JSON.stringify(req.body)
      );

      const rfq: Rfq = createExchangeRequestBody.message;
      await rfq.verify();

      const offering = offeringsRepository.getOffering(rfq.data.offeringId);
      if (!offering) {
        throw new Error('Offering not found');
      }

      // TODO: Uncomment when we have valid pex input
      // await rfq.verifyOfferingRequirements(offering);

      if (createExchangeRequestBody.replyTo) {
        exchangesToReplyTo.set(rfq.metadata.exchangeId, createExchangeRequestBody.replyTo);
      }

      exchangeIdToExchange.set(rfq.metadata.exchangeId, { rfq });

      // Asynchronously send a quote in response to the RFQ
      setImmediate(async () => {
        try {
          await replyWithQuote(rfq.metadata.from, rfq.metadata.exchangeId);
        } catch (error) {
          console.error('Error in replyWithQuote:', error);
        }
      });

      res.status(202).send();
    } catch (error) {
      console.error('Error in POST /exchanges:', error);
      res.status(400).json({
        error: error.message || 'RFQ does not satisfy an available offering',
      });
    }
  });

  /**
   * PUT /exchanges/:id
   * Updates an existing exchange with a new message (Order or Cancel)
   */
  router.put('/:id', async (req, res) => {
    const exchangeId = req.params.id;
    console.log(`PUT /exchanges/${exchangeId}`);

    try {
      const updateExchangeRequestBody = UpdateExchangeRequestBody.fromJSONString(
        JSON.stringify(req.body)
      );

      const message = updateExchangeRequestBody.message;

      if (message instanceof Order) {
        const orderMessage = message as Order;
        await orderMessage.verify();

        // Process the order asynchronously
        await processOrder(orderMessage);
      } else if (message instanceof Cancel) {
        const cancelMessage = message as Cancel;
        await cancelMessage.verify();

        // Process the cancellation
        await replyWithClose(cancelMessage.metadata.from, cancelMessage.metadata.exchangeId, false);
      }

      res.status(202).send();
    } catch (error) {
      console.error('Error in PUT /exchanges/:id', error);
      res.status(400).json({ error: error.message || 'Invalid request' });
    }
  });

  /**
   * Processes an Order message by sending a series of status updates
   * @param orderMessage - The Order message to process
   */
  async function processOrder(orderMessage: Order) {
    try {
      await replyWithOrderInstructions(orderMessage.metadata.from, orderMessage.metadata.exchangeId);

      const statuses = [
        "PAYIN_INITIATED",
        "PAYIN_SETTLED",
        "PAYOUT_INITIATED",
        "PAYOUT_SETTLED",
      ];

      for (const status of statuses) {
        await delay(500);
        await replyWithOrderStatus(orderMessage.metadata.from, orderMessage.metadata.exchangeId, status);
      }

      await delay(500);
      await replyWithClose(orderMessage.metadata.from, orderMessage.metadata.exchangeId);
    } catch (error) {
      console.error('Error processing order:', error);
    }
  }

  /**
   * Sends a Quote message in response to an RFQ
   * @param to - The recipient DID
   * @param exchangeId - The exchange ID
   */
  async function replyWithQuote(to: string, exchangeId: string) {
    console.log('Replying with quote');

    const quoteData: QuoteData = {
      expiresAt: new Date(Date.now() + 3600 * 1000).toISOString(), // Expires in 1 hour
      payin: {
        currencyCode: 'BTC',
        subtotal: '1000.00',
        total: '1001.00',
        fee: null,
      },
      payout: {
        currencyCode: 'KES',
        subtotal: '1000.00',
        total: '1001.00',
        fee: null,
      },
      payoutUnitsPerPayinUnit: '1.0',
    };

    const quote = Quote.create(to, bearerDid.did.uri, exchangeId, quoteData);

    await quote.sign(bearerDid);
    await quote.verify();

    const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
    exchangeIdToExchange.set(exchangeId, { ...existingExchange, quote });

    await sendReply(exchangeId, new ReplyToRequestBody(quote));
  }

  /**
   * Sends OrderInstructions in response to an Order
   * @param to - The recipient DID
   * @param exchangeId - The exchange ID
   */
  async function replyWithOrderInstructions(to: string, exchangeId: string) {
    console.log('Replying with order instructions');

    const orderInstructionsData: OrderInstructionsData = {
      payin: {
        link: 'http://tbd.website/payin',
        instruction: 'Payin instruction',
      },
      payout: {
        link: 'http://tbd.website/payout',
        instruction: 'Payout instruction',
      },
    };

    const orderInstructions = OrderInstructions.create(
      to,
      bearerDid.did.uri,
      exchangeId,
      orderInstructionsData
    );

    await orderInstructions.sign(bearerDid);
    await orderInstructions.verify();

    const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
    exchangeIdToExchange.set(exchangeId, {
      ...existingExchange,
      orderInstructions,
    });

    await sendReply(exchangeId, new ReplyToRequestBody(orderInstructions));
  }

  /**
   * Sends an OrderStatus update
   * @param to - The recipient DID
   * @param exchangeId - The exchange ID
   * @param status - The status to update
   */
  async function replyWithOrderStatus(to: string, exchangeId: string, status: string) {
    console.log(`Replying with order status: ${status}`);

    const orderStatusData: OrderStatusData = {
      status,
    };

    const orderStatus = OrderStatus.create(to, bearerDid.did.uri, exchangeId, orderStatusData);

    await orderStatus.sign(bearerDid);
    await orderStatus.verify();

    const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
    const updatedOrderStatuses = existingExchange.orderStatuses || [];
    updatedOrderStatuses.push(orderStatus);
    exchangeIdToExchange.set(exchangeId, {
      ...existingExchange,
      orderStatuses: updatedOrderStatuses,
    });

    await sendReply(exchangeId, new ReplyToRequestBody(orderStatus));
  }

  /**
   * Sends a Close message to indicate the exchange is closed
   * @param to - The recipient DID
   * @param exchangeId - The exchange ID
   * @param success - Whether the exchange was successful
   */
  async function replyWithClose(to: string, exchangeId: string, success = true) {
    console.log('Replying with close');

    const closeData: CloseData = {
      reason: null,
      success,
    };

    const close = Close.create(to, bearerDid.did.uri, exchangeId, closeData);

    await close.sign(bearerDid);
    await close.verify();

    const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
    exchangeIdToExchange.set(exchangeId, { ...existingExchange, close });

    await sendReply(exchangeId, new ReplyToRequestBody(close));
  }

  /**
   * Sends a reply message to the client
   * @param exchangeId - The exchange ID
   * @param message - The message to send
   */
  async function sendReply(exchangeId: string, replyToRequestBody: ReplyToRequestBody) {
    const replyTo = exchangesToReplyTo.get(exchangeId);
    if (replyTo) {
      try {
        await axios.post(replyTo, replyToRequestBody.toJSONString(), {
          headers: {
            'Content-Type': 'application/json; charset=utf-8',
          },
        });
      } catch (error) {
        console.error('Error sending reply:', error);
      }
    }
  }

  return router;
}