import { expect } from "chai";
import OfferingVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json" assert { type: "json" };
import BalanceVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-balance.json" assert { type: "json" };
import RfqVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-rfq.json" assert { type: "json" };
import { Offering } from "../src/resources/offering";
import { PortableDid } from "../src/portable-did";
import { BearerDid } from "../src/bearer-did";
import { Balance } from "../src/resources/balance";
import { Rfq } from "../src/messages/rfq";
import { CreateRfqData } from "../src/wasm/mappings";

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
    it("should parse", () => {
      const input = OfferingVector.input;
      const offering = Offering.fromJSONString(input);
      expect(offering.metadata).to.deep.equal(OfferingVector.output.metadata);
      expect(offering.data).to.deep.equal(OfferingVector.output.data);
      expect(offering.signature).to.equal(OfferingVector.output.signature);

      const offeringJSONString = offering.toJSONString();
      const offeringJSON = JSON.parse(offeringJSONString);
      expect(offeringJSON).to.deep.equal(OfferingVector.output);

      offering.verify();
    });

    it("should create, sign, and verify", () => {
      const offering = Offering.create(
        OfferingVector.output.metadata.from,
        OfferingVector.output.data,
        OfferingVector.output.metadata.protocol
      );

      offering.sign(bearerDID);
      offering.verify();
    });
  });

  describe("balance", () => {
    it("should parse", () => {
      const input = BalanceVector.input;
      const balance = Balance.fromJSONString(input);
      expect(balance.metadata).to.deep.equal(BalanceVector.output.metadata);
      expect(balance.data).to.deep.equal(BalanceVector.output.data);
      expect(balance.signature).to.equal(BalanceVector.output.signature);

      const balanceJSONString = balance.toJSONString();
      const balanceJSON = JSON.parse(balanceJSONString);
      expect(balanceJSON).to.deep.equal(BalanceVector.output);

      balance.verify();
    });

    it("should create, sign, and verify", () => {
      const balance = Balance.create(
        BalanceVector.output.metadata.from,
        BalanceVector.output.data,
        BalanceVector.output.metadata.protocol
      );

      balance.sign(bearerDID);
      balance.verify();
    });
  });

  describe("rfq", () => {
    it("should parse", () => {
      const input = RfqVector.input;
      const rfq = Rfq.fromJSONString(input);
      expect(rfq.metadata).to.deep.equal(RfqVector.output.metadata);
      expect(rfq.data).to.deep.equal(RfqVector.output.data);
      expect(rfq.signature).to.equal(RfqVector.output.signature);

      const balanceJSONString = rfq.toJSONString();
      const balanceJSON = JSON.parse(balanceJSONString);
      expect(balanceJSON).to.deep.equal(RfqVector.output);

      rfq.verify();
    });

    it("should create, sign, and verify", () => {
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
      rfq.verify();
    });
  });
});
