import { expect } from "chai";
import OfferingVector from "../../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json" assert { type: 'json' }
import { Offering } from "../src/resources/offering";

describe('test vectors', () => {
  it("should parse offering", () => {
    const input = OfferingVector.input
    const offering = Offering.fromJsonString(input)

    expect(offering.inner).to.deep.equal(OfferingVector.output)

    offering.verify()
  })
});