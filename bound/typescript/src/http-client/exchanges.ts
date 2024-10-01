import { BearerDid } from "../bearer-did";
import { tbdexError } from "../errors";
import { Cancel } from "../messages/cancel";
import { Close } from "../messages/close";
import { Order } from "../messages/order";
import { OrderInstructions } from "../messages/order-instructions";
import { OrderStatus } from "../messages/order-status";
import { Quote } from "../messages/quote";
import { Rfq } from "../messages/rfq";
import wasm from "../wasm";

export const createExchange = async (
  rfq: Rfq,
  replyTo?: string
): Promise<void> => {
  await wasm.create_exchange(rfq.toWASM(), replyTo);
};

export const submitOrder = async (order: Order): Promise<void> => {
  await wasm.submit_order(order.toWASM());
};

export const submitCancel = async (cancel: Cancel): Promise<void> => {
  await wasm.submit_order(cancel.toWASM());
};

export const getExchange = async (
  pfiDidUri: string,
  bearerDid: BearerDid,
  exchangeId: string
): Promise<Exchange> => {
  const wasmExchange = await wasm.get_exchange(
    pfiDidUri,
    bearerDid.toWASM(),
    exchangeId
  );
  return Exchange.fromWASM(wasmExchange);
};

export type GetExchangeIdsQueryParams = {
  paginationLimit?: number;
  paginationOffset?: number;
};

export const getExchangeIds = async (
  pfiDidUri: string,
  requestorDid: BearerDid,
  options?: GetExchangeIdsQueryParams
): Promise<string[]> => {
  return await wasm.get_exchange_ids(
    pfiDidUri,
    requestorDid.toWASM(),
    options?.paginationOffset ? BigInt(options.paginationOffset) : undefined,
    options?.paginationLimit ? BigInt(options.paginationLimit) : undefined
  );
};

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
