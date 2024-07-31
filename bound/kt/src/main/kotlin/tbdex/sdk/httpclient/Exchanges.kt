package tbdex.sdk.httpclient

import tbdex.sdk.messages.*
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.ExchangeData as RustCoreExchange
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitCancel as rustCoreSubmitCancel
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchanges as rustCoreGetExchanges
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
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

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
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    rustCoreCreateExchange(rfq.rustCoreRfq, replyTo)
}

fun submitOrder(order: Order) {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    rustCoreSubmitOrder(order.rustCoreOrder)
}

fun submitCancel(cancel: Cancel) {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    rustCoreSubmitCancel(cancel.rustCoreCancel)
}

fun getExchange(pfiDidUri: String, bearerDid: BearerDid, exchangeId: String): Exchange {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    val rustCoreExchange = rustCoreGetExchange(pfiDidUri, bearerDid.rustCoreBearerDid, exchangeId)
    return Exchange.fromRustCore(rustCoreExchange)
}

fun getExchanges(pfiDidUri: String, bearerDid: BearerDid): List<String> {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    return rustCoreGetExchanges(pfiDidUri, bearerDid.rustCoreBearerDid)
}



