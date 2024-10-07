import { Router } from 'express';
import axios from 'axios';
import {
  BearerDid,
  PortableDid,
  Offering,
  GetExchangeResponseBody,
  tbdexError,
  CreateExchangeRequestBody,
  UpdateExchangeRequestBody,
  ReplyToRequestBody,
  TbdexError,
  Quote,
  OrderInstructions,
  OrderStatus,
  Close,
  Rfq,
  Order,
  Cancel,
  Message,
  GetExchangesResponseBody
  // Status,
  // PaymentInstruction,
} from 'tbdex';
// import { CloseData, OrderInstructionsData, OrderStatusData, PaymentInstruction, QuoteData } from 'tbdex/dist/wasm/generated-mappings';


// import { GetExchangesResponseBody } from 'tbdex/dist/http/exchanges/';

interface Exchange {
  rfq?: Message;
  quote?: Message;
  order?: Message;
  orderInstructions?: Message;
  cancel?: Message;
  close?: Message;
  orderStatuses?: Message[];
}

function createExchangesRouter(
  bearerDid: BearerDid,
  offeringsRepository: any // Replace 'any' with the actual type
) {
  const router = Router();

  const exchangesToReplyTo: Map<string, string> = new Map();
  const exchangeIdToExchange: Map<string, Exchange> = new Map();

  router.get('/', (req, res) => {
    console.log('GET /exchanges');

    const offset = parseInt(req.query['page[offset]'] as string) || 0;
    const limit = parseInt(req.query['page[limit]'] as string) || 10;
    const exchangeIds = Array.from(exchangeIdToExchange.keys());
    const paginatedExchanges = exchangeIds.slice(offset, offset + limit);

    const bearerDID = BearerDid.fromPortableDID(
      PortableDid.fromJSONString(
        '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
      )
    );
    // const responseBody = new GetExchangesResponseBody(paginatedExchanges);

    res.type('application/json');
    // res.send(responseBody.toJSONString());
    res.send('hi');
  });

  // router.get('/:id', (req, res) => {
  //   const exchangeId = req.params.id;
  //   console.log(`GET /exchanges/${exchangeId}`);

  //   const exchange = exchangeIdToExchange.get(exchangeId);
  //   if (!exchange) {
  //     res.status(404).send('Exchange not found');
  //     return;
  //   }

  //   const messages = [
  //     exchange.rfq,
  //     exchange.quote,
  //     exchange.order,
  //     exchange.orderInstructions,
  //     exchange.cancel,
  //     exchange.close,
  //     ...(exchange.orderStatuses || []),
  //   ].filter(Boolean);

  //   const responseBody = new GetExchangeResponseBody(messages as Message[]);

  //   res.type('application/json');
  //   res.send(responseBody.toJSONString());
  // });

  // router.post('/', (req, res) => {
  //   console.log('POST /exchanges');

  //   const requestBody = CreateExchangeRequestBody.fromJSONString(req.body);
  //   const rfq = requestBody.message as Rfq;

  //   rfq.verify();

  //   try {
  //     rfq.verifyOfferingRequirements(
  //       offeringsRepository.getOffering(rfq.data.offeringId)
  //     );
  //   } catch (e) {
  //     res.status(400);
  //     const errorResponseBody = new Error(
  //       'RFQ does not satisfy an available offering'
  //     );
  //     res.send(errorResponseBody);
  //     return;
  //   }

  //   if (requestBody.replyTo) {
  //     exchangesToReplyTo.set(rfq.metadata.exchangeId, requestBody.replyTo);
  //   }
  //   exchangeIdToExchange.set(rfq.metadata.exchangeId, { rfq });

  //   res.status(202).send();

  //   setTimeout(() => {
  //     replyWithQuote(rfq.metadata.from, rfq.metadata.exchangeId);
  //   }, 500);
  // });

  // router.put('/:id', (req, res) => {
  //   const exchangeId = req.params.id;
  //   console.log(`PUT /exchanges/${exchangeId}`);

  //   const updateExchangeRequestBody = UpdateExchangeRequestBody.fromJSONString(
  //     req.body
  //   );
  //   const message = updateExchangeRequestBody.message;

  //   if (message instanceof Order) {
  //     const orderMessage = message as Order;
  //     // Simulate order execution
  //     orderMessage.verify();

  //     setTimeout(() => {
  //       replyWithOrderInstructions(
  //         orderMessage.metadata.from,
  //         orderMessage.metadata.exchangeId
  //       );
  //       setTimeout(() => {
  //         replyWithOrderStatus(
  //           orderMessage.metadata.from,
  //           orderMessage.metadata.exchangeId,
  //           'PAYIN_INITIATED'
  //         );
  //         setTimeout(() => {
  //           replyWithOrderStatus(
  //             orderMessage.metadata.from,
  //             orderMessage.metadata.exchangeId,
  //             'PAYIN_SETTLED'
  //           );
  //           setTimeout(() => {
  //             replyWithOrderStatus(
  //               orderMessage.metadata.from,
  //               orderMessage.metadata.exchangeId,
  //               'PAYOUT_INITIATED'
  //             );
  //             setTimeout(() => {
  //               replyWithOrderStatus(
  //                 orderMessage.metadata.from,
  //                 orderMessage.metadata.exchangeId,
  //                 'PAYOUT_SETTLED'
  //               );
  //               setTimeout(() => {
  //                 replyWithClose(
  //                   orderMessage.metadata.from,
  //                   orderMessage.metadata.exchangeId
  //                 );
  //               }, 500);
  //             }, 500);
  //           }, 500);
  //         }, 500);
  //       }, 500);
  //     }, 500);
  //   } else if (message instanceof Cancel) {
  //     const cancelMessage = message as Cancel;
  //     // Simulate cancel
  //     cancelMessage.verify();

  //     setTimeout(() => {
  //       replyWithClose(
  //         cancelMessage.metadata.from,
  //         cancelMessage.metadata.exchangeId,
  //         false
  //       );
  //     }, 500);
  //   }

  //   res.status(202).send();
  // });

  // function replyWithQuote(to: string, exchangeId: string) {
  //   const quoteData: QuoteData = {
  //       expiresAt: '2024-08-02T04:26:08.239Z',
  //       payin: {
  //         currencyCode: 'BTC',
  //         subtotal: '1000.00',
  //         total: '1001.00',
  //       },
  //       payout: {
  //         currencyCode: 'KES',
  //         subtotal: '1000.00',
  //         total: '1001.00',
  //       },
  //       payoutUnitsPerPayinUnit: '1.0',
  //   };
    
  //   const quote = Quote.create(to, bearerDid.did.uri, exchangeId, quoteData);

  //   quote.sign(bearerDid);
  //   quote.verify();

  //   console.log('Replying with quote');

  //   const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  //   exchangeIdToExchange.set(exchangeId, { ...existingExchange, quote });

  //   const replyTo = exchangesToReplyTo.get(exchangeId);
  //   if (replyTo) {
  //     replyRequest(replyTo, new ReplyToRequestBody(quote));
  //   }
  // }

  // function replyWithOrderInstructions(to: string, exchangeId: string) {
  //   const payInPaymentInstruction: PaymentInstruction = {link: "http://tbd.website/payin", instruction: "payin instruction"};
  //   const payOutPaymentInstruction: PaymentInstruction = {link: "http://tbd.website/payin", instruction: "payin instruction"};


  //   const orderInstructionsData: OrderInstructionsData = {payin: payInPaymentInstruction, payout: payOutPaymentInstruction};

  //   const orderInstructions = OrderInstructions.create(to, bearerDid.did.uri, exchangeId, orderInstructionsData)

  //   orderInstructions.sign(bearerDid);
  //   orderInstructions.verify();

  //   console.log('Replying with orderInstructions');

  //   const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  //   exchangeIdToExchange.set(exchangeId, {
  //     ...existingExchange,
  //     orderInstructions,
  //   });

  //   const replyTo = exchangesToReplyTo.get(exchangeId);
  //   if (replyTo) {
  //     replyRequest(replyTo, new ReplyToRequestBody(orderInstructions));
  //   }
  // }

  // function replyWithOrderStatus(
  //   to: string,
  //   exchangeId: string,
  //   status: string
  // ) {

  //   const OrderStatusData: OrderStatusData = {
  //     status: status,
  //   }

  //   const orderStatus = OrderStatus.create(to, bearerDid.did.uri, exchangeId, OrderStatusData);

  //   orderStatus.sign(bearerDid);
  //   orderStatus.verify();

  //   console.log(`Replying with order status ${status}`);

  //   const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  //   const updatedOrderStatuses = existingExchange.orderStatuses || [];
  //   updatedOrderStatuses.push(orderStatus);
  //   exchangeIdToExchange.set(exchangeId, {
  //     ...existingExchange,
  //     orderStatuses: updatedOrderStatuses,
  //   });

  //   const replyTo = exchangesToReplyTo.get(exchangeId);
  //   if (replyTo) {
  //     replyRequest(replyTo, new ReplyToRequestBody(orderStatus));
  //   }
  // }

  // function replyWithClose(
  //   to: string,
  //   exchangeId: string,
  //   success: boolean = true
  // ) {

    
  //   const closeData: CloseData = {
  //     success: true,
  //   }

  //   const close = Close.create(
  //     to,
  //     bearerDid.did.uri,
  //     exchangeId,
  //     closeData
  //   );

  //   close.sign(bearerDid);
  //   close.verify();

  //   console.log('Replying with close');

  //   const existingExchange = exchangeIdToExchange.get(exchangeId) || {};
  //   exchangeIdToExchange.set(exchangeId, { ...existingExchange, close });

  //   const replyTo = exchangesToReplyTo.get(exchangeId);
  //   if (replyTo) {
  //     replyRequest(replyTo, new ReplyToRequestBody(close));
  //   }
  // }

  // function replyRequest(replyTo: string, body: ReplyToRequestBody) {
  //   axios
  //     .post(replyTo, body.toJSONString(), {
  //       headers: {
  //         'Content-Type': 'application/json; charset=utf-8',
  //       },
  //     })
  //     .catch((error) => {
  //       console.error('Error in replyRequest:', error);
  //     });
  // }

  return router;
}

export default createExchangesRouter;