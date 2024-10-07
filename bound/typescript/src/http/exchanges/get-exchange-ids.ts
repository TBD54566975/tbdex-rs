export class GetExchangesResponseBody {
  readonly data: string[];

  constructor(data: string[]) {
    this.data = data;
  }

  static fromJSONString = (json: string): GetExchangesResponseBody => {
    const obj = JSON.parse(json);
    return new GetExchangesResponseBody(obj.data);
  };

  toJSONString = (): string => {
    return JSON.stringify({ data: this.data });
  };
}
