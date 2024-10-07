import { Cancel } from "./cancel";
import { Close } from "./close";
import { Order } from "./order";
import { OrderInstructions } from "./order-instructions";
import { OrderStatus } from "./order-status";
import { Quote } from "./quote";
import { Rfq } from "./rfq";

export type Message =
  | Rfq
  | Quote
  | Order
  | OrderInstructions
  | Cancel
  | OrderStatus
  | Close;

export type MessageMetadata = {
  createdAt: string;
  exchangeId: string;
  externalId?: string;
  from: string;
  id: string;
  kind: MessageKind;
  protocol: string;
  to: string;
};

export const RFQ_KIND = "rfq" as const;
export const QUOTE_KIND = "quote" as const;
export const ORDER_KIND = "order" as const;
export const ORDER_INSTRUCTIONS_KIND = "orderinstructions" as const;
export const ORDER_STATUS_KIND = "orderstatus" as const;
export const CLOSE_KIND = "close" as const;
export const CANCEL_KIND = "cancel" as const;

export type MessageKind =
  | typeof RFQ_KIND
  | typeof QUOTE_KIND
  | typeof ORDER_KIND
  | typeof ORDER_INSTRUCTIONS_KIND
  | typeof ORDER_STATUS_KIND
  | typeof CLOSE_KIND
  | typeof CANCEL_KIND;
