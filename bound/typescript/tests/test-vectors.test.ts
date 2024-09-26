import { expect } from "chai";
import OfferingVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json" assert { type: "json" };
import { Offering } from "../src/resources/offering";
import { PortableDid } from "../src/portable-did";
import { BearerDid } from "../src/bearer-did";

describe("test vectors", () => {
  it("should parse offering", () => {
    const input = OfferingVector.input;
    const offering = Offering.fromJSONString(input);
    expect(offering.metadata).to.deep.equal(OfferingVector.output.metadata);
    expect(offering.data).to.deep.equal(OfferingVector.output.data);
    expect(offering.signature).to.equal(OfferingVector.output.signature);

    const offeringJSONString = offering.toJSONString();
    const offeringJSON = JSON.parse(offeringJSONString);
    expect(offeringJSON).to.deep.equal(OfferingVector.output);

    offering.verify();

    const createdOffering = Offering.create(
      OfferingVector.output.metadata.from,
      OfferingVector.output.data,
      OfferingVector.output.metadata.protocol
    );
    console.log(createdOffering);

    const portableDID = PortableDid.fromJSONString(
      '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"0ddITLHzlgIVRqPMZ0efciIWi0EWZ20fQnfzP0s-jhs"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiIwZGRJVExIemxnSVZScVBNWjBlZmNpSVdpMEVXWjIwZlFuZnpQMHMtamhzIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"obPbLmD7oKBPs2Z-RlY4X8tCyMz7h6KOyxuuWXoTU9U","x":"n20lmtdADeRx6BZ6rNAm33vtx5B8fQQnqLS6NjKUg5A"}]}'
    );
    console.log(portableDID);
    const bearerDID = BearerDid.fromPortableDID(portableDID);
    console.log(bearerDID);

    // TODO call sign
    // createdOffering.verify();
  });
});
