import { Router } from 'express';
import axios from 'axios';
import {
  PortableDid,
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
  PaymentInstruction,
  QuoteData,
  OrderInstructionsData,
  OrderStatusData,
  CloseData,
  ORDER_STATUS_KIND
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

// const portableDID: PortableDid = JSON.parse(
//   '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
// );

// const bearerDid = BearerDid.fromPortableDID(portableDID);

export default function exchangesRouter(bearerDid: BearerDid, offeringsRepository: OfferingRepository) {

const router = Router();

const exchangesToReplyTo: Map<string, string> = new Map();
const exchangeIdToExchange: Map<string, Exchange> = new Map();

router.get('/', (req, res) => {
  console.log('GET /exchanges');

  const offset = parseInt(req.query['page[offset]'] as string) || 0;
  const limit = parseInt(req.query['page[limit]'] as string) || 10;
  const exchangeIds = Array.from(exchangeIdToExchange.keys());
  const paginatedExchanges = exchangeIds.slice(offset, offset + limit);

  const responseBody = new GetExchangesResponseBody(paginatedExchanges);

  res.type('application/json');
  res.send(responseBody.toJSONString());
});

router.get('/:id', (req, res) => {
  console.log("get exchange id")
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

router.post('/', async (req, res) => {
  console.log('POST /exchanges');

  try {
    const createExchangeRequestBody = CreateExchangeRequestBody.fromJSONString(JSON.stringify(req.body));

    const rfq: Rfq = createExchangeRequestBody.message;
    await rfq.verify();

    console.log("before offer req")

    const offering = offeringsRepository.getOffering(rfq.data.offeringId)
    console.log("got the offering:")
    console.log(offering)

    // TODO: Add this back in
    // await rfq.verifyOfferingRequirements(offering);

    console.log("after offer req")


    console.log("1.1")
    // TODO:
    if (createExchangeRequestBody.replyTo) {
      exchangesToReplyTo.set(rfq.metadata.exchangeId, createExchangeRequestBody.replyTo);
    }

    console.log("1.2")

    exchangeIdToExchange.set(rfq.metadata.exchangeId, { rfq });

    console.log("1.3")
    
    setTimeout(() => {
      console.log("2.0")
      replyWithQuote(rfq.metadata.from, rfq.metadata.exchangeId);
      console.log("2.1")
    }, 500);

    res.status(202).send();
  } catch (e) {
    console.log(e)
    res.status(400).json({
      error: 'RFQ does not satisfy an available offering',
    });
  }
});

router.put('/:id', (req, res) => {
  const exchangeId = req.params.id;
  console.log(`PUT /exchanges/${exchangeId}`);

  const updateExchangeRequestBody = UpdateExchangeRequestBody.fromJSONString(
    req.body
  );
  const message = updateExchangeRequestBody.message;

  if (message instanceof Order) {
    const orderMessage = message as Order;
    // Simulate order execution
    orderMessage.verify();

    setTimeout(() => {
      replyWithOrderInstructions(
        orderMessage.metadata.from,
        orderMessage.metadata.exchangeId
      );
      setTimeout(() => {
        replyWithOrderStatus(
          orderMessage.metadata.from,
          orderMessage.metadata.exchangeId,
          "PAYIN_INITIATED"
        );
        setTimeout(() => {
          replyWithOrderStatus(
            orderMessage.metadata.from,
            orderMessage.metadata.exchangeId,
            "PAYIN_SETTLED"
          );
          setTimeout(() => {
            replyWithOrderStatus(
              orderMessage.metadata.from,
              orderMessage.metadata.exchangeId,
              "PAYOUT_INITIATED"
            );
            setTimeout(() => {
              replyWithOrderStatus(
                orderMessage.metadata.from,
                orderMessage.metadata.exchangeId,
                "PAYOUT_SETTLED"
              );
              setTimeout(() => {
                replyWithClose(
                  orderMessage.metadata.from,
                  orderMessage.metadata.exchangeId
                );
              }, 500);
            }, 500);
          }, 500);
        }, 500);
      }, 500);
    }, 500);
  } else if (message instanceof Cancel) {
    const cancelMessage = message as Cancel;
    // Simulate cancel
    cancelMessage.verify();

    setTimeout(() => {
      replyWithClose(
        cancelMessage.metadata.from,
        cancelMessage.metadata.exchangeId,
        false
      );
    }, 500);
  }

  res.status(202).send();
});

function replyWithQuote(to: string, exchangeId: string) {
  console.log("reply with quote start")
  const quoteData: QuoteData = {
    expiresAt: '2024-08-02T04:26:08.239Z',
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

  const quote = Quote.create(
    to,
    bearerDid.did.uri,
    exchangeId,
    quoteData
  );

  console.log("befoer with quote sign")
  quote.sign(bearerDid);
  console.log("befoer with quote verify")
  quote.verify();

  console.log('Replying with quote');

  const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  exchangeIdToExchange.set(exchangeId, { ...existingExchange, quote });

  const replyTo = exchangesToReplyTo.get(exchangeId);
  if (replyTo) {
    replyRequest(replyTo, new ReplyToRequestBody(quote));
  }
}

function replyWithOrderInstructions(to: string, exchangeId: string) {
  console.log('Replying with orderInstructions');

  const orderInstructionsData: OrderInstructionsData = {
    payin: {
      link: 'http://tbd.website/payin',
      instruction: 'payin instruction',
    },
    payout: {
      link: 'http://tbd.website/payout',
      instruction: 'payout instruction',
    },
  };

  const orderInstructions = OrderInstructions.create(
    to,
    bearerDid.did.uri,
    exchangeId,
    orderInstructionsData
  );

  orderInstructions.sign(bearerDid);
  orderInstructions.verify();

  const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  exchangeIdToExchange.set(exchangeId, {
    ...existingExchange,
    orderInstructions,
  });

  const replyTo = exchangesToReplyTo.get(exchangeId);
  if (replyTo) {
    replyRequest(replyTo, new ReplyToRequestBody(orderInstructions));
  }
}

function replyWithOrderStatus(
  to: string,
  exchangeId: string,
  status: string
) {
  const orderStatusData: OrderStatusData = {
    status: status,
  };

  const orderStatus = OrderStatus.create(
    to,
    bearerDid.did.uri,
    exchangeId,
    orderStatusData
  );

  orderStatus.sign(bearerDid);
  orderStatus.verify();

  console.log(`Replying with order status ${status}`);

  const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  const updatedOrderStatuses = existingExchange.orderStatuses || [];
  updatedOrderStatuses.push(orderStatus);
  exchangeIdToExchange.set(exchangeId, {
    ...existingExchange,
    orderStatuses: updatedOrderStatuses,
  });

  const replyTo = exchangesToReplyTo.get(exchangeId);
  if (replyTo) {
    replyRequest(replyTo, new ReplyToRequestBody(orderStatus));
  }
}

function replyWithClose(
  to: string,
  exchangeId: string,
  success: boolean = true
) {

  console.log('Replying with close');

  const closeData: CloseData = {
    reason: null,
    success: success,
  };

  const close = Close.create(
    to,
    bearerDid.did.uri,
    exchangeId,
    closeData
  );

  close.sign(bearerDid);
  close.verify();


  const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  exchangeIdToExchange.set(exchangeId, { ...existingExchange, close });

  const replyTo = exchangesToReplyTo.get(exchangeId);
  if (replyTo) {
    replyRequest(replyTo, new ReplyToRequestBody(close));
  }
}

function replyRequest(replyTo: string, body: ReplyToRequestBody) {
  axios
    .post(replyTo, body.toJSONString(), {
      headers: {
        'Content-Type': 'application/json; charset=utf-8',
      },
    })
    .catch((error) => {
      console.error('Error in replyRequest:', error);
    });
}

  return router;
}


// export default createExchangesRouter;
// export default router;
