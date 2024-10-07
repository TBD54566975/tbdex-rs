import { BearerDid } from "../dids/bearer-did";
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
  await wasm.create_exchange(rfq.toJSONString(), replyTo);
};

export const submitOrder = async (order: Order): Promise<void> => {
  await wasm.submit_order(order.toJSONString());
};

export const submitCancel = async (cancel: Cancel): Promise<void> => {
  await wasm.submit_cancel(cancel.toJSONString());
};

export const getExchange = async (
  pfiDidUri: string,
  bearerDid: BearerDid,
  exchangeId: string
): Promise<Exchange> => {
  const json = await wasm.get_exchange(
    pfiDidUri,
    bearerDid.toWASM(),
    exchangeId
  );
  return Exchange.fromJSONString(json);
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

  static fromJSONString = (json: string): Exchange => {
    const obj = JSON.parse(json);

    const rfq = new Rfq(
      obj.rfq.metadata,
      obj.rfq.data,
      obj.rfq.privateData,
      obj.rfq.signature
    );
    const quote = obj.quote
      ? new Quote(obj.quote.metadata, obj.quote.data, obj.quote.signature)
      : undefined;
    const order = obj.order
      ? new Order(obj.order.metadata, obj.order.data, obj.order.signature)
      : undefined;
    const orderInstructions = obj.orderInstructions
      ? new OrderInstructions(
          obj.orderInstructions.metadata,
          obj.orderInstructions.data,
          obj.orderInstructions.signature
        )
      : undefined;
    const cancel = obj.cancel
      ? new Cancel(obj.cancel.metadata, obj.cancel.data, obj.cancel.signature)
      : undefined;
    const orderStatuses = obj.orderStatuses
      ? obj.orderStatuses.map(
          (x: OrderStatus) => new OrderStatus(x.metadata, x.data, x.signature)
        )
      : undefined;
    const close = obj.close
      ? new Close(obj.close.metadata, obj.close.data, obj.close.signature)
      : undefined;

    return new Exchange(
      rfq,
      quote,
      order,
      orderInstructions,
      cancel,
      orderStatuses,
      close
    );
  };
}
