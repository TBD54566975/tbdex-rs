import {
  CANCEL_KIND,
  CLOSE_KIND,
  Message,
  ORDER_INSTRUCTIONS_KIND,
  ORDER_KIND,
  ORDER_STATUS_KIND,
  QUOTE_KIND,
  RFQ_KIND,
} from "../../messages";
import { Cancel } from "../../messages/cancel";
import { Close } from "../../messages/close";
import { Order } from "../../messages/order";
import { OrderInstructions } from "../../messages/order-instructions";
import { OrderStatus } from "../../messages/order-status";
import { Quote } from "../../messages/quote";
import { Rfq, RfqPrivateData } from "../../messages/rfq";

export class GetExchangeResponseBody {
  readonly data: Message[];

  constructor(data: Message[]) {
    this.data = data;
  }

  static fromJSONString = (json: string): GetExchangeResponseBody => {
    const obj = JSON.parse(json);
    return new GetExchangeResponseBody(
      obj.data.map((x: Message) => {
        switch (x.metadata.kind) {
          case RFQ_KIND:
            return new Rfq(
              x.metadata,
              (x as Rfq).data,
              (x as Rfq).privateData as RfqPrivateData,
              x.signature
            );
          case QUOTE_KIND:
            return new Quote(x.metadata, (x as Quote).data, x.signature);
          case ORDER_KIND:
            return new Order(x.metadata, (x as Order).data, x.signature);
          case ORDER_INSTRUCTIONS_KIND:
            return new OrderInstructions(
              x.metadata,
              (x as OrderInstructions).data,
              x.signature
            );
          case ORDER_STATUS_KIND:
            return new OrderStatus(
              x.metadata,
              (x as OrderStatus).data,
              x.signature
            );
          case CLOSE_KIND:
            return new Close(x.metadata, (x as Close).data, x.signature);
          case CANCEL_KIND:
            return new Cancel(x.metadata, (x as Cancel).data, x.signature);
          default:
            throw new Error(`unknown message kind ${x.metadata.kind}`);
        }
      })
    );
  };

  toJSONString = (): string => {
    return JSON.stringify({ data: this.data });
  };
}
