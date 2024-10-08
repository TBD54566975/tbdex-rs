import { expect } from "chai";
import OfferingVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json" assert { type: "json" };
import BalanceVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-balance.json" assert { type: "json" };
import RfqVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-rfq.json" assert { type: "json" };
import RfqOmitPrivateDataVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-rfq-omit-private-data.json" assert { type: "json" };
import QuoteVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-quote.json" assert { type: "json" };
import OrderInstructionsVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-orderinstructions.json" assert { type: "json" };
import OrderVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-order.json" assert { type: "json" };
import CancelVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-cancel.json" assert { type: "json" };
import OrderStatusVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-orderstatus.json" assert { type: "json" };
import CloseVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-close.json" assert { type: "json" };
import GetExchangeResponseBodyVector from "./vectors/http/exchanges/get-exchange.json" assert { type: "json" };
import GetExchangeIdsResponseBodyVector from "./vectors/http/exchanges/get-exchange-ids.json" assert { type: "json" };
import CreateExchangeRequestBodyVector from "./vectors/http/exchanges/create-exchange.json" assert { type: "json" };
import UpdateExchangeRequestBodyOrderVector from "./vectors/http/exchanges/update-exchange-order.json" assert { type: "json" };
import UpdateExchangeRequestBodyCancelVector from "./vectors/http/exchanges/update-exchange-cancel.json" assert { type: "json" };
import ReplyToRequestBodyQuoteVector from "./vectors/http/exchanges/reply-to-quote.json" assert { type: "json" };
import ReplyToRequestBodyOrderInstructionsVector from "./vectors/http/exchanges/reply-to-orderinstructions.json" assert { type: "json" };
import ReplyToRequestBodyOrderStatusVector from "./vectors/http/exchanges/reply-to-orderstatus.json" assert { type: "json" };
import ReplyToRequestBodyCloseVector from "./vectors/http/exchanges/reply-to-close.json" assert { type: "json" };
import GetOfferingsResponseBodyVector from "./vectors/http/get-offerings.json" assert { type: "json" };
import { Offering } from "../src/resources/offering";
import { Balance } from "../src/resources/balance";
import { CreateRfqData, Rfq } from "../src/messages/rfq";
import { Quote } from "../src/messages/quote";
import { Order } from "../src/messages/order";
import { Cancel } from "../src/messages/cancel";
import { OrderStatus } from "../src/messages/order-status";
import { Close } from "../src/messages/close";
import { Message } from "../src/messages";
import { Resource } from "../src/resources";
import { OrderInstructions } from "../src/messages/order-instructions";
import {
  CreateExchangeRequestBody,
  GetExchangeResponseBody,
  GetExchangesResponseBody,
  ReplyToRequestBody,
  UpdateExchangeRequestBody,
} from "../src/http/exchanges";
import { GetOfferingsResponseBody } from "../src/http/offerings";
import { BearerDid } from "../src/dids/bearer-did";
import { PortableDid } from "../src/dids/portable-did";

describe("test vectors", () => {
  let bearerDID: BearerDid;

  before(() => {
    const portableDID: PortableDid = JSON.parse(
      '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
    );
    bearerDID = BearerDid.fromPortableDID(portableDID);
  });

  describe("resources", () => {
    describe("offering", () => {
      it("should parse", async () => {
        const input = OfferingVector.input;
        const offering = Offering.fromJSONString(input);
        expect(offering.metadata).to.deep.equal(OfferingVector.output.metadata);
        expect(offering.data).to.deep.equal(OfferingVector.output.data);
        expect(offering.signature).to.equal(OfferingVector.output.signature);

        const offeringJSONString = offering.toJSONString();
        const offeringJSON = JSON.parse(offeringJSONString);
        expect(offeringJSON).to.deep.equal(OfferingVector.output);

        await offering.verify();
      });

      it("should create, sign, and verify", async () => {
        const offering = Offering.create(
          OfferingVector.output.metadata.from,
          OfferingVector.output.data,
          OfferingVector.output.metadata.protocol
        );

        offering.sign(bearerDID);
        await offering.verify();
      });

      it("should be instance of resource", async () => {
        const resource: Resource = Offering.fromJSONString(
          OfferingVector.input
        );
        expect(resource instanceof Offering).to.be.true;
        expect(resource instanceof Balance).to.be.false;
      });
    });

    describe("balance", () => {
      it("should parse", async () => {
        const input = BalanceVector.input;
        const balance = Balance.fromJSONString(input);
        expect(balance.metadata).to.deep.equal(BalanceVector.output.metadata);
        expect(balance.data).to.deep.equal(BalanceVector.output.data);
        expect(balance.signature).to.equal(BalanceVector.output.signature);

        const balanceJSONString = balance.toJSONString();
        const balanceJSON = JSON.parse(balanceJSONString);
        expect(balanceJSON).to.deep.equal(BalanceVector.output);

        await balance.verify();
      });

      it("should create, sign, and verify", async () => {
        const balance = Balance.create(
          BalanceVector.output.metadata.from,
          BalanceVector.output.data,
          BalanceVector.output.metadata.protocol
        );

        balance.sign(bearerDID);
        await balance.verify();
      });

      it("should be instance of resource", async () => {
        const resource: Resource = Balance.fromJSONString(BalanceVector.input);
        expect(resource instanceof Balance).to.be.true;
        expect(resource instanceof Offering).to.be.false;
      });
    });
  });

  describe("messages", () => {
    describe("rfq", () => {
      it("should parse", async () => {
        const input = RfqVector.input;
        const rfq = Rfq.fromJSONString(input);
        expect(rfq.metadata).to.deep.equal(RfqVector.output.metadata);
        expect(rfq.data).to.deep.equal(RfqVector.output.data);
        expect(rfq.signature).to.equal(RfqVector.output.signature);

        const rfqJSONString = rfq.toJSONString();
        const rfqJSON = JSON.parse(rfqJSONString);
        expect(rfqJSON).to.deep.equal(RfqVector.output);

        await rfq.verify();
      });

      it("should parse with private data omitted", async () => {
        const input = RfqOmitPrivateDataVector.input;
        const rfq = Rfq.fromJSONString(input);
        expect(rfq.metadata).to.deep.equal(
          RfqOmitPrivateDataVector.output.metadata
        );
        expect(rfq.data).to.deep.equal(RfqOmitPrivateDataVector.output.data);
        expect(rfq.signature).to.equal(
          RfqOmitPrivateDataVector.output.signature
        );

        const rfqJSONString = rfq.toJSONString();
        const rfqJSON = JSON.parse(rfqJSONString);
        expect(rfqJSON).to.deep.equal(RfqOmitPrivateDataVector.output);

        await rfq.verify();
      });

      it("should create, sign, and verify", async () => {
        const createRfqData: CreateRfqData = {
          claims: RfqVector.output.privateData.claims,
          offeringId: RfqVector.output.data.offeringId,
          payin: {
            amount: RfqVector.output.data.payin.amount,
            kind: RfqVector.output.data.payin.kind,
            paymentDetails: RfqVector.output.privateData.payin.paymentDetails,
          },
          payout: {
            kind: RfqVector.output.data.payout.kind,
            paymentDetails: RfqVector.output.privateData.payout.paymentDetails,
          },
        };

        const rfq = Rfq.create(
          RfqVector.output.metadata.to,
          RfqVector.output.metadata.from,
          createRfqData,
          RfqVector.output.metadata.protocol
        );

        rfq.sign(bearerDID);
        await rfq.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = Rfq.fromJSONString(RfqVector.input);
        expect(message instanceof Rfq).to.be.true;
        expect(message instanceof OrderStatus).to.be.false;
      });
    });

    describe("quote", () => {
      it("should parse", async () => {
        const input = QuoteVector.input;
        const quote = Quote.fromJSONString(input);
        expect(quote.metadata).to.deep.equal(QuoteVector.output.metadata);
        expect(quote.data).to.deep.equal(QuoteVector.output.data);
        expect(quote.signature).to.equal(QuoteVector.output.signature);

        const quoteJSONString = quote.toJSONString();
        const quoteJSON = JSON.parse(quoteJSONString);
        expect(quoteJSON).to.deep.equal(QuoteVector.output);

        quote.verify();
      });

      it("should create, sign, and verify", async () => {
        const quote = Quote.create(
          QuoteVector.output.metadata.to,
          QuoteVector.output.metadata.from,
          QuoteVector.output.metadata.exchangeId,
          QuoteVector.output.data,
          QuoteVector.output.metadata.protocol
        );

        quote.sign(bearerDID);
        await quote.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = Quote.fromJSONString(QuoteVector.input);
        expect(message instanceof Quote).to.be.true;
        expect(message instanceof OrderStatus).to.be.false;
      });
    });

    describe("order", () => {
      it("should parse", async () => {
        const input = OrderVector.input;
        const order = Order.fromJSONString(input);
        expect(order.metadata).to.deep.equal(OrderVector.output.metadata);
        expect(order.data).to.deep.equal(OrderVector.output.data);
        expect(order.signature).to.equal(OrderVector.output.signature);

        const orderJSONString = order.toJSONString();
        const orderJSON = JSON.parse(orderJSONString);
        expect(orderJSON).to.deep.equal(OrderVector.output);

        await order.verify();
      });

      it("should create, sign, and verify", async () => {
        const order = Order.create(
          OrderVector.output.metadata.to,
          OrderVector.output.metadata.from,
          OrderVector.output.metadata.exchangeId,
          OrderVector.output.metadata.protocol
        );

        order.sign(bearerDID);
        await order.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = Order.fromJSONString(OrderVector.input);
        expect(message instanceof Order).to.be.true;
        expect(message instanceof OrderStatus).to.be.false;
      });
    });

    describe("order instructions", () => {
      it("should parse", async () => {
        const input = OrderInstructionsVector.input;
        const orderInstructions = OrderInstructions.fromJSONString(input);
        expect(orderInstructions.metadata).to.deep.equal(
          OrderInstructionsVector.output.metadata
        );
        expect(orderInstructions.data).to.deep.equal(
          OrderInstructionsVector.output.data
        );
        expect(orderInstructions.signature).to.equal(
          OrderInstructionsVector.output.signature
        );

        const orderInstructionsJSONString = orderInstructions.toJSONString();
        const orderInstructionsJSON = JSON.parse(orderInstructionsJSONString);
        expect(orderInstructionsJSON).to.deep.equal(
          OrderInstructionsVector.output
        );

        await orderInstructions.verify();
      });

      it("should create, sign, and verify", async () => {
        const orderInstructions = OrderInstructions.create(
          OrderInstructionsVector.output.metadata.to,
          OrderInstructionsVector.output.metadata.from,
          OrderInstructionsVector.output.metadata.exchangeId,
          OrderInstructionsVector.output.data,
          OrderInstructionsVector.output.metadata.protocol
        );

        orderInstructions.sign(bearerDID);
        await orderInstructions.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = OrderInstructions.fromJSONString(
          OrderInstructionsVector.input
        );
        expect(message instanceof OrderInstructions).to.be.true;
        expect(message instanceof OrderStatus).to.be.false;
      });
    });

    describe("cancel", () => {
      it("should parse", async () => {
        const input = CancelVector.input;
        const cancel = Cancel.fromJSONString(input);
        expect(cancel.metadata).to.deep.equal(CancelVector.output.metadata);
        expect(cancel.data).to.deep.equal(CancelVector.output.data);
        expect(cancel.signature).to.equal(CancelVector.output.signature);

        const cancelJSONString = cancel.toJSONString();
        const cancelJSON = JSON.parse(cancelJSONString);
        expect(cancelJSON).to.deep.equal(CancelVector.output);

        await cancel.verify();
      });

      it("should create, sign, and verify", async () => {
        const cancel = Cancel.create(
          CancelVector.output.metadata.to,
          CancelVector.output.metadata.from,
          CancelVector.output.metadata.exchangeId,
          CancelVector.output.data,
          CancelVector.output.metadata.protocol
        );

        cancel.sign(bearerDID);
        await cancel.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = Cancel.fromJSONString(CancelVector.input);
        expect(message instanceof Cancel).to.be.true;
        expect(message instanceof OrderStatus).to.be.false;
      });
    });

    describe("order status", () => {
      it("should parse", async () => {
        const input = OrderStatusVector.input;
        const orderStatus = OrderStatus.fromJSONString(input);
        expect(orderStatus.metadata).to.deep.equal(
          OrderStatusVector.output.metadata
        );
        expect(orderStatus.data).to.deep.equal(OrderStatusVector.output.data);
        expect(orderStatus.signature).to.equal(
          OrderStatusVector.output.signature
        );

        const orderStatusJSONString = orderStatus.toJSONString();
        const orderStatusJSON = JSON.parse(orderStatusJSONString);
        expect(orderStatusJSON).to.deep.equal(OrderStatusVector.output);

        await orderStatus.verify();
      });

      it("should create, sign, and verify", async () => {
        const orderStatus = OrderStatus.create(
          OrderStatusVector.output.metadata.to,
          OrderStatusVector.output.metadata.from,
          OrderStatusVector.output.metadata.exchangeId,
          OrderStatusVector.output.data,
          OrderStatusVector.output.metadata.protocol
        );

        orderStatus.sign(bearerDID);
        await orderStatus.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = OrderStatus.fromJSONString(
          OrderStatusVector.input
        );
        expect(message instanceof OrderStatus).to.be.true;
        expect(message instanceof Cancel).to.be.false;
      });
    });

    describe("close", () => {
      it("should parse", async () => {
        const input = CloseVector.input;
        const close = Close.fromJSONString(input);
        expect(close.metadata).to.deep.equal(CloseVector.output.metadata);
        expect(close.data).to.deep.equal(CloseVector.output.data);
        expect(close.signature).to.equal(CloseVector.output.signature);

        const closeJSONString = close.toJSONString();
        const closeJSON = JSON.parse(closeJSONString);
        expect(closeJSON).to.deep.equal(CloseVector.output);

        await close.verify();
      });

      it("should create, sign, and verify", async () => {
        const close = Close.create(
          CloseVector.output.metadata.to,
          CloseVector.output.metadata.from,
          CloseVector.output.metadata.exchangeId,
          CloseVector.output.data,
          CloseVector.output.metadata.protocol
        );

        close.sign(bearerDID);
        await close.verify();
      });

      it("should be instance of message", async () => {
        const message: Message = Close.fromJSONString(CloseVector.input);
        expect(message instanceof Close).to.be.true;
        expect(message instanceof Cancel).to.be.false;
      });
    });
  });

  describe("http", () => {
    describe("exchanges", () => {
      describe("get exchange response body", () => {
        it("should parse", async () => {
          const getExchangeResponseBody =
            GetExchangeResponseBody.fromJSONString(
              GetExchangeResponseBodyVector.input
            );
          expect(getExchangeResponseBody.data.length).to.equal(
            GetExchangeResponseBodyVector.output.data.length
          );

          const rfq = getExchangeResponseBody.data[0];
          expect(rfq instanceof Rfq).to.be.true;
          expect(rfq.metadata).to.deep.equal(
            GetExchangeResponseBodyVector.output.data[0].metadata
          );
          expect(rfq.data).to.deep.equal(
            GetExchangeResponseBodyVector.output.data[0].data
          );
          expect(rfq.signature).to.equal(
            GetExchangeResponseBodyVector.output.data[0].signature
          );
          await (rfq as Rfq).verify();

          const quote = getExchangeResponseBody.data[1];
          expect(quote instanceof Quote).to.be.true;
          expect(quote.metadata).to.deep.equal(
            GetExchangeResponseBodyVector.output.data[1].metadata
          );
          expect(quote.data).to.deep.equal(
            GetExchangeResponseBodyVector.output.data[1].data
          );
          expect(quote.signature).to.equal(
            GetExchangeResponseBodyVector.output.data[1].signature
          );
          await (quote as Quote).verify();
        });
      });

      describe("get exchange ids response body", () => {
        it("should parse", async () => {
          const getExchangesResponseBody =
            GetExchangesResponseBody.fromJSONString(
              GetExchangeIdsResponseBodyVector.input
            );
          expect(getExchangesResponseBody.data.length).to.equal(
            GetExchangeIdsResponseBodyVector.output.data.length
          );
          expect(getExchangesResponseBody.data[0]).to.equal(
            GetExchangeIdsResponseBodyVector.output.data[0]
          );
          expect(getExchangesResponseBody.data[1]).to.equal(
            GetExchangeIdsResponseBodyVector.output.data[1]
          );
        });
      });

      describe("create exchange request body", () => {
        it("should parse", async () => {
          const createExchangeRequestBody =
            CreateExchangeRequestBody.fromJSONString(
              CreateExchangeRequestBodyVector.input
            );
          expect(createExchangeRequestBody.message.metadata).to.deep.equal(
            CreateExchangeRequestBodyVector.output.message.metadata
          );
          expect(createExchangeRequestBody.message.data).to.deep.equal(
            CreateExchangeRequestBodyVector.output.message.data
          );
          expect(createExchangeRequestBody.message.signature).to.deep.equal(
            CreateExchangeRequestBodyVector.output.message.signature
          );
          expect(createExchangeRequestBody.replyTo).to.deep.equal(
            CreateExchangeRequestBodyVector.output.replyTo
          );

          await createExchangeRequestBody.message.verify();
        });
      });

      describe("update exchange request body", () => {
        it("should parse order", async () => {
          const updateExchangeRequestBody =
            UpdateExchangeRequestBody.fromJSONString(
              UpdateExchangeRequestBodyOrderVector.input
            );

          expect(updateExchangeRequestBody.message instanceof Order).to.be.true;
          expect(updateExchangeRequestBody.message.metadata).to.deep.equal(
            UpdateExchangeRequestBodyOrderVector.output.message.metadata
          );
          expect(updateExchangeRequestBody.message.data).to.deep.equal(
            UpdateExchangeRequestBodyOrderVector.output.message.data
          );
          expect(updateExchangeRequestBody.message.signature).to.deep.equal(
            UpdateExchangeRequestBodyOrderVector.output.message.signature
          );

          await updateExchangeRequestBody.message.verify();
        });

        it("should parse cancel", async () => {
          const updateExchangeRequestBody =
            UpdateExchangeRequestBody.fromJSONString(
              UpdateExchangeRequestBodyCancelVector.input
            );

          expect(updateExchangeRequestBody.message instanceof Cancel).to.be
            .true;
          expect(updateExchangeRequestBody.message.metadata).to.deep.equal(
            UpdateExchangeRequestBodyCancelVector.output.message.metadata
          );
          expect(updateExchangeRequestBody.message.data).to.deep.equal(
            UpdateExchangeRequestBodyCancelVector.output.message.data
          );
          expect(updateExchangeRequestBody.message.signature).to.deep.equal(
            UpdateExchangeRequestBodyCancelVector.output.message.signature
          );

          await updateExchangeRequestBody.message.verify();
        });
      });

      describe("reply to request body", () => {
        it("should parse quote", async () => {
          const replyToRequestBody = ReplyToRequestBody.fromJSONString(
            ReplyToRequestBodyQuoteVector.input
          );

          expect(replyToRequestBody.message instanceof Quote).to.be.true;
          expect(replyToRequestBody.message.metadata).to.deep.equal(
            ReplyToRequestBodyQuoteVector.output.message.metadata
          );
          expect(replyToRequestBody.message.data).to.deep.equal(
            ReplyToRequestBodyQuoteVector.output.message.data
          );
          expect(replyToRequestBody.message.signature).to.deep.equal(
            ReplyToRequestBodyQuoteVector.output.message.signature
          );

          await replyToRequestBody.message.verify();
        });

        it("should parse order instructions", async () => {
          const replyToRequestBody = ReplyToRequestBody.fromJSONString(
            ReplyToRequestBodyOrderInstructionsVector.input
          );

          expect(replyToRequestBody.message instanceof OrderInstructions).to.be
            .true;
          expect(replyToRequestBody.message.metadata).to.deep.equal(
            ReplyToRequestBodyOrderInstructionsVector.output.message.metadata
          );
          expect(replyToRequestBody.message.data).to.deep.equal(
            ReplyToRequestBodyOrderInstructionsVector.output.message.data
          );
          expect(replyToRequestBody.message.signature).to.deep.equal(
            ReplyToRequestBodyOrderInstructionsVector.output.message.signature
          );

          await replyToRequestBody.message.verify();
        });

        it("should parse order status", async () => {
          const replyToRequestBody = ReplyToRequestBody.fromJSONString(
            ReplyToRequestBodyOrderStatusVector.input
          );

          expect(replyToRequestBody.message instanceof OrderStatus).to.be.true;
          expect(replyToRequestBody.message.metadata).to.deep.equal(
            ReplyToRequestBodyOrderStatusVector.output.message.metadata
          );
          expect(replyToRequestBody.message.data).to.deep.equal(
            ReplyToRequestBodyOrderStatusVector.output.message.data
          );
          expect(replyToRequestBody.message.signature).to.deep.equal(
            ReplyToRequestBodyOrderStatusVector.output.message.signature
          );

          await replyToRequestBody.message.verify();
        });

        it("should parse close", async () => {
          const replyToRequestBody = ReplyToRequestBody.fromJSONString(
            ReplyToRequestBodyCloseVector.input
          );

          expect(replyToRequestBody.message instanceof Close).to.be.true;
          expect(replyToRequestBody.message.metadata).to.deep.equal(
            ReplyToRequestBodyCloseVector.output.message.metadata
          );
          expect(replyToRequestBody.message.data).to.deep.equal(
            ReplyToRequestBodyCloseVector.output.message.data
          );
          expect(replyToRequestBody.message.signature).to.deep.equal(
            ReplyToRequestBodyCloseVector.output.message.signature
          );

          await replyToRequestBody.message.verify();
        });
      });
    });

    describe("offerings", () => {
      it("should parse", async () => {
        const getOfferingsResponseBody =
          GetOfferingsResponseBody.fromJSONString(
            GetOfferingsResponseBodyVector.input
          );

        expect(getOfferingsResponseBody.data[0].metadata).to.deep.equal(
          GetOfferingsResponseBodyVector.output.data[0].metadata
        );
        expect(getOfferingsResponseBody.data[0].data).to.deep.equal(
          GetOfferingsResponseBodyVector.output.data[0].data
        );
        expect(getOfferingsResponseBody.data[0].signature).to.deep.equal(
          GetOfferingsResponseBodyVector.output.data[0].signature
        );

        await getOfferingsResponseBody.data[0].verify();
      });
    });
  });
});
