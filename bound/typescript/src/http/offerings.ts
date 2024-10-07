import { Offering } from "../resources/offering";

export class GetOfferingsResponseBody {
  readonly data: Offering[];

  constructor(data: Offering[]) {
    this.data = data;
  }

  static fromJSONString = (json: string): GetOfferingsResponseBody => {
    const obj = JSON.parse(json);
    return new GetOfferingsResponseBody(
      obj.data.map(
        (x: Offering) => new Offering(x.metadata, x.data, x.signature)
      )
    );
  };

  toJSONString = (): string => {
    return JSON.stringify({ data: this.data });
  };
}
