package tbdex.sdk.httpclient

import tbdex.sdk.messages.*
import tbdex.sdk.rust.ExchangeData as RustCoreExchange
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitClose as rustCoreSubmitClose
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchanges as rustCoreGetExchanges

data class Exchange(
    val rfq: Rfq,
    val quote: Quote? = null,
    val order: Order? = null,
    val orderStatuses: List<OrderStatus>? = null,
    val close: Close? = null
) {
    companion object {
        fun fromRustCore(rustCoreExchange: RustCoreExchange): Exchange {
            return Exchange(
                Rfq(rustCoreExchange.rfq),
                rustCoreExchange.quote?.let { Quote(it) },
                rustCoreExchange.order?.let { Order(it) },
                rustCoreExchange.orderStatuses?.let { it -> it.map { OrderStatus(it) } },
                rustCoreExchange.close?.let { Close(it) },
            )
        }
    }

    fun toJson(): String {
        return this.rustCoreRfq.toJson()
    }
}

fun createExchange(rfq: Rfq, replyTo: String? = null) {
    rustCoreCreateExchange(rfq.rustCoreRfq, replyTo)
}

fun submitOrder(order: Order) {
    rustCoreSubmitOrder(order.rustCoreOrder)
}

fun submitClose(close: Close) {
    rustCoreSubmitClose(close.rustCoreClose)
}

fun getExchange(pfiDidUri: String, bearerDid: BearerDid, exchangeId: String): Exchange {
    val rustCoreExchange = rustCoreGetExchange(pfiDidUri, bearerDid.rustCoreBearerDid, exchangeId)
    return Exchange.fromRustCore(rustCoreExchange)
}

fun getExchanges(pfiDidUri: String, bearerDid: BearerDid): List<String> {
    return rustCoreGetExchanges(pfiDidUri, bearerDid.rustCoreBearerDid)
}