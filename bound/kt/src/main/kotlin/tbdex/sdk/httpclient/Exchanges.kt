package tbdex.sdk.httpclient

import tbdex.sdk.messages.*
import tbdex.sdk.rust.Exchange as RustCoreExchange
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitClose as rustCoreSubmitClose
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchanges as rustCoreGetExchanges
import tbdex.sdk.rust.CreateExchangeRequestBody as RustCoreCreateExchangeRequestBody

class Exchange {
    val rfq: Rfq
    val quote: Quote?
    val order: Order?
    val orderStatuses: List<OrderStatus>?
    val close: Close?

    internal val rustCoreExchange: RustCoreExchange

    constructor(rustCoreExchange: RustCoreExchange) {
        this.rustCoreExchange = rustCoreExchange

        val data = this.rustCoreExchange.getData()
        this.rfq = Rfq(data.rfq)
        this.quote = data.quote?.let { Quote(it) }
        this.order = data.order?.let { Order(it) }
        this.orderStatuses = data.orderStatuses?.map { OrderStatus(it) }
        this.close = data.close?.let { Close(it) }
    }

    fun toJson(): String {
        return this.rustCoreExchange.toJson()
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
    return Exchange(rustCoreExchange)
}

fun getExchanges(pfiDidUri: String, bearerDid: BearerDid): List<String> {
    return rustCoreGetExchanges(pfiDidUri, bearerDid.rustCoreBearerDid)
}

class CreateExchangeRequestBody {
    val rfq: Rfq
    val replyTo: String?

    constructor(json: String) {
        val rustCoreCreateExchangeRequestBody = RustCoreCreateExchangeRequestBody.fromJsonString(json)
        val data = rustCoreCreateExchangeRequestBody.getData()
        this.rfq = Rfq(data.rfq)
        this.replyTo = data.replyTo
    }
}