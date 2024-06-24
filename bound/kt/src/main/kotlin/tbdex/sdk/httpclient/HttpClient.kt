package tbdex.sdk.httpclient

import tbdex.sdk.messages.Close
import tbdex.sdk.messages.Order
import tbdex.sdk.messages.Rfq
import tbdex.sdk.resources.Balance
import tbdex.sdk.resources.Offering
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings
import tbdex.sdk.rust.getBalances as rustCoreGetBalances
import tbdex.sdk.rust.createExchange as rustCoreCreateExchange
import tbdex.sdk.rust.submitOrder as rustCoreSubmitOrder
import tbdex.sdk.rust.submitClose as rustCoreSubmitClose
import tbdex.sdk.rust.getExchange as rustCoreGetExchange
import tbdex.sdk.rust.getExchanges as rustCoreGetExchanges

fun getOfferings(pfiDidUri: String): List<Offering> {
    val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
    return rustCoreOfferings.map { Offering(it) }
}

fun getBalances(pfiDidUri: String, requestorBearerDid: BearerDid): List<Balance> {
    val rustCoreBalances = rustCoreGetBalances(pfiDidUri, requestorBearerDid.rustCoreBearerDid)
    return rustCoreBalances.map { Balance(it) }
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

fun getExchange(pfiDidUri: String, requestorBearerDid: BearerDid, exchangeId: String): Exchange {
    return rustCoreGetExchange(pfiDidUri, requestorBearerDid.rustCoreBearerDid, exchangeId)
}

fun getExchanges(pfiDidUri: String, requestorBearerDid: BearerDid): List<String> {
    return rustCoreGetExchanges(pfiDidUri, requestorBearerDid.rustCoreBearerDid)
}