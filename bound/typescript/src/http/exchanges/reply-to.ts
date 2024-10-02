import {
  CLOSE_KIND,
  ORDER_INSTRUCTIONS_KIND,
  ORDER_STATUS_KIND,
  QUOTE_KIND,
} from "../../messages";
import { Close } from "../../messages/close";
import { OrderInstructions } from "../../messages/order-instructions";
import { OrderStatus } from "../../messages/order-status";
import { Quote } from "../../messages/quote";

export type ReplyToMessage = Quote | OrderInstructions | OrderStatus | Close;

export class ReplyToRequestBody {
  readonly message: ReplyToMessage;

  constructor(message: ReplyToMessage) {
    this.message = message;
  }

  static fromJSONString = (json: string): ReplyToRequestBody => {
    const obj = JSON.parse(json);

    let message: ReplyToMessage;
    switch (obj.message.metadata.kind) {
      case QUOTE_KIND:
        obj.message = obj.message as Quote;
        message = new Quote(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      case ORDER_INSTRUCTIONS_KIND:
        obj.message = obj.message as OrderInstructions;
        message = new OrderInstructions(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      case ORDER_STATUS_KIND:
        obj.message = obj.message as OrderStatus;
        message = new OrderStatus(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      case CLOSE_KIND:
        obj.message = obj.message as Close;
        message = new Close(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      default:
        throw new Error(`unknown message kind ${obj.message.metadata.kind}`);
    }

    return new ReplyToRequestBody(message);
  };

  toJSONString = (): string => {
    return JSON.stringify({ message: this.message });
  };
}
