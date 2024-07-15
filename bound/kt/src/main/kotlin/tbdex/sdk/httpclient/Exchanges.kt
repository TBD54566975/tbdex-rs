package tbdex.sdk.httpclient

import tbdex.sdk.messages.*
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.ExchangeData as RustCoreExchange
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitCancel as rustCoreSubmitCancel
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchanges as rustCoreGetExchanges

data class Exchange(
    val rfq: Rfq,
    val quote: Quote? = null,
    val order: Order? = null,
    val cancel: Cancel? = null,
    val orderStatuses: List<OrderStatus>? = null,
    val close: Close? = null
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromRustCore(rustCoreExchange: RustCoreExchange): Exchange {
            return Exchange(
                Rfq(rustCoreExchange.rfq),
                rustCoreExchange.quote?.let { Quote(it) },
                rustCoreExchange.order?.let { Order(it) },
                rustCoreExchange.cancel?.let { Cancel(it) },
                rustCoreExchange.orderStatuses?.let { it -> it.map { OrderStatus(it) } },
                rustCoreExchange.close?.let { Close(it) },
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



