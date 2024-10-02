import { tbdexError } from "../errors";
import { Offering } from "../resources/offering";
import wasm from "../wasm";

export class GetOfferingsResponseBody {
  readonly data: Offering[];

  constructor(data: Offering[]) {
    this.data = data;
  }

  static fromJSONString = (json: string): GetOfferingsResponseBody => {
    try {
      const getOfferingsResponseBody = JSON.parse(json);
      const offerings: Offering[] = [];
      for (const offeringObject of getOfferingsResponseBody.data) {
        offerings.push(
          new Offering(
            offeringObject.metadata,
            offeringObject.data,
            offeringObject.signature
          )
        );
      }
      return new GetOfferingsResponseBody(offerings);
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toJSONString = (): string => {
    try {
      return JSON.stringify(this);
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
