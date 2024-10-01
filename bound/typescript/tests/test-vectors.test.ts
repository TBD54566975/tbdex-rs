import { expect } from "chai";
import OfferingVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json" assert { type: "json" };
import BalanceVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-balance.json" assert { type: "json" };
import RfqVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-rfq.json" assert { type: "json" };
import RfqOmitPrivateDataVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-rfq-omit-private-data.json" assert { type: "json" };
import QuoteVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-quote.json" assert { type: "json" };
import OrderVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-order.json" assert { type: "json" };
import CancelVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-cancel.json" assert { type: "json" };
import OrderStatusVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-orderstatus.json" assert { type: "json" };
import CloseVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-close.json" assert { type: "json" };
import { Offering } from "../src/resources/offering";
import { PortableDid } from "../src/portable-did";
import { BearerDid } from "../src/bearer-did";
import { Balance } from "../src/resources/balance";
import { Rfq } from "../src/messages/rfq";
import { CreateRfqData } from "../src/wasm/generated-mappings";
import { Quote } from "../src/messages/quote";
import { Order } from "../src/messages/order";
import { Cancel } from "../src/messages/cancel";
import { OrderStatus } from "../src/messages/order-status";
import { Close } from "../src/messages/close";

describe("test vectors", () => {
  let bearerDID: BearerDid;

  before(() => {
    bearerDID = BearerDid.fromPortableDID(
      PortableDid.fromJSONString(
        '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
      )
    );
  });

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
  });

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
      expect(rfq.signature).to.equal(RfqOmitPrivateDataVector.output.signature);

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
  });

  describe("order instructions", () => {
    it("should parse", async () => {
      // todo create test vector
    });

    it("should create, sign, and verify", async () => {
      // todo create test vector
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
  });
});
