import { Rfq } from "../../messages/rfq";

export class CreateExchangeRequestBody {
  readonly message: Rfq;
  readonly replyTo?: string;

  constructor(message: Rfq, replyTo?: string) {
    this.message = message;
    this.replyTo = replyTo;
  }

  static fromJSONString = (json: string): CreateExchangeRequestBody => {
    const obj = JSON.parse(json);
    const rfq = new Rfq(
      obj.message.metadata,
      obj.message.data,
      obj.message.privateData,
      obj.message.signature
    );
    return new CreateExchangeRequestBody(rfq, obj.replyTo);
  };

  toJSONString = (): string => {
    return JSON.stringify({
      message: this.message,
      replyTo: this.replyTo,
    });
  };
}
