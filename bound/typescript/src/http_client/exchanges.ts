import { tbdexError } from "../errors";
import { Cancel } from "../messages/cancel";
import { Close } from "../messages/close";
import { Order } from "../messages/order";
import { OrderInstructions } from "../messages/order-instructions";
import { OrderStatus } from "../messages/order-status";
import { Quote } from "../messages/quote";
import { Rfq } from "../messages/rfq";
import wasm from "../wasm";

export class Exchange {
  readonly rfq: Rfq;
  readonly quote?: Quote;
  readonly order?: Order;
  readonly orderInstructions?: OrderInstructions;
  readonly cancel?: Cancel;
  readonly orderStatuses?: OrderStatus[];
  readonly close?: Close;

  constructor(
    rfq: Rfq,
    quote?: Quote,
    order?: Order,
    orderInstructions?: OrderInstructions,
    cancel?: Cancel,
    orderStatuses?: OrderStatus[],
    close?: Close
  ) {
    this.rfq = rfq;
    this.quote = quote;
    this.order = order;
    this.orderInstructions = orderInstructions;
    this.cancel = cancel;
    this.orderStatuses = orderStatuses;
    this.close = close;
  }

  static fromWASM = (wasmExchange: wasm.WasmExchange): Exchange => {
    try {
      return new Exchange(
        Rfq.fromWASM(wasmExchange.rfq),
        wasmExchange.quote ? Quote.fromWASM(wasmExchange.quote) : undefined,
        wasmExchange.order ? Order.fromWASM(wasmExchange.order) : undefined,
        wasmExchange.order_instructions
          ? OrderInstructions.fromWASM(wasmExchange.order_instructions)
          : undefined,
        wasmExchange.cancel ? Cancel.fromWASM(wasmExchange.cancel) : undefined,
        wasmExchange.order_statuses
          ? wasmExchange.order_statuses.map(OrderStatus.fromWASM)
          : undefined,
        wasmExchange.close ? Close.fromWASM(wasmExchange.close) : undefined
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };

  toWASM = (): wasm.WasmExchange => {
    try {
      return new wasm.WasmExchange(
        this.rfq.toWASM(),
        this.quote?.toWASM(),
        this.order?.toWASM(),
        this.orderInstructions?.toWASM(),
        this.cancel?.toWASM(),
        this.orderStatuses?.map((os) => os.toWASM()),
        this.close?.toWASM()
      );
    } catch (error) {
      throw tbdexError(error);
    }
  };
}
