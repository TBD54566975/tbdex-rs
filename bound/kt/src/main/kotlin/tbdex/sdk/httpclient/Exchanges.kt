package tbdex.sdk.httpclient

import tbdex.sdk.messages.*
import tbdex.sdk.rust.GetExchangeIdsQueryParamsData as RustCoreGetExchangeIdsQueryParams
import tbdex.sdk.rust.ExchangeData as RustCoreExchange
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitCancel as rustCoreSubmitCancel
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchangeIds as rustCoreGetExchangeIds
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class Exchange(
    val rfq: Rfq,
    val quote: Quote? = null,
    val order: Order? = null,
    val orderInstructions: OrderInstructions? = null,
    val cancel: Cancel? = null,
    val orderStatuses: List<OrderStatus>? = null,
    val close: Close? = null
) {
    companion object {
        internal fun fromRustCore(rustCoreExchange: RustCoreExchange): Exchange {
            return Exchange(
                Rfq.fromRustCoreRfq(rustCoreExchange.rfq),
                rustCoreExchange.quote?.let { Quote.fromRustCoreQuote(it) },
                rustCoreExchange.order?.let { Order.fromRustCoreOrder(it) },
                rustCoreExchange.orderInstructions?.let { OrderInstructions.fromRustCoreOrderInstructions(it) },
                rustCoreExchange.cancel?.let { Cancel.fromRustCoreCancel(it) },
                rustCoreExchange.orderStatuses?.let { it -> it.map { OrderStatus.fromRustCoreOrderStatus(it) } },
                rustCoreExchange.close?.let { Close.fromRustCoreClose(it) },
            )
        }
    }
}

fun createExchange(rfq: Rfq, replyTo: String? = null) {
    rustCoreCreateExchange(rfq.rustCoreRfq, replyTo)
}

fun submitOrder(order: Order) {
    rustCoreSubmitOrder(order.rustCoreOrder)
}

fun submitCancel(cancel: Cancel) {
    rustCoreSubmitCancel(cancel.rustCoreCancel)
}

fun getExchange(pfiDidUri: String, bearerDid: BearerDid, exchangeId: String): Exchange {
    val rustCoreExchange = rustCoreGetExchange(pfiDidUri, RustCoreBearerDid.fromWeb5(bearerDid), exchangeId)
    return Exchange.fromRustCore(rustCoreExchange)
}

typealias GetExchangeIdsQueryParams = RustCoreGetExchangeIdsQueryParams

fun getExchangeIds(pfiDidUri: String, bearerDid: BearerDid, queryParams: GetExchangeIdsQueryParams? = null): List<String> {
    return rustCoreGetExchangeIds(pfiDidUri, RustCoreBearerDid.fromWeb5(bearerDid), queryParams)
}



