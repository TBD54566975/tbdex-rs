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
