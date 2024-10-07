import { CANCEL_KIND, ORDER_KIND } from "../../messages";
import { Cancel } from "../../messages/cancel";
import { Order } from "../../messages/order";

export type WalletUpdateMessage = Order | Cancel;

export class UpdateExchangeRequestBody {
  readonly message: WalletUpdateMessage;

  constructor(message: WalletUpdateMessage) {
    this.message = message;
  }

  static fromJSONString = (json: string): UpdateExchangeRequestBody => {
    const obj = JSON.parse(json);

    let message: WalletUpdateMessage;
    switch (obj.message.metadata.kind) {
      case ORDER_KIND:
        obj.message = obj.message as Order;
        message = new Order(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      case CANCEL_KIND:
        obj.message = obj.message as Cancel;
        message = new Cancel(
          obj.message.metadata,
          obj.message.data,
          obj.message.signature
        );
        break;
      default:
        throw new Error(`unknown message kind ${obj.message.metadata.kind}`);
    }

    return new UpdateExchangeRequestBody(message);
  };

  toJSONString = (): string => {
    return JSON.stringify({ message: this.message });
  };
}
