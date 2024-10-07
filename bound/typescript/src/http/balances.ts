import { Balance } from "../resources/balance";

export class GetBalancesResponseBody {
  readonly data: Balance[];

  constructor(data: Balance[]) {
    this.data = data;
  }

  static fromJSONString = (json: string): GetBalancesResponseBody => {
    const obj = JSON.parse(json);
    return new GetBalancesResponseBody(
      obj.data.map((x: Balance) => new Balance(x.metadata, x.data, x.signature))
    );
  };

  toJSONString = (): string => {
    return JSON.stringify({ data: this.data });
  };
}
