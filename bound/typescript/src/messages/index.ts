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

export type MessageKind =
  | "rfq"
  | "quote"
  | "order"
  | "orderinstructions"
  | "orderstatus"
  | "close"
  | "cancel";
