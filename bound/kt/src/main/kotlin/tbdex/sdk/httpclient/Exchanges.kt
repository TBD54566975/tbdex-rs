package tbdex.sdk.httpclient

import tbdex.sdk.TbdexException
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
    try {
        rustCoreCreateExchange(rfq.rustCoreRfq, replyTo)
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}

fun submitOrder(order: Order) {
    try {
        rustCoreSubmitOrder(order.rustCoreOrder)
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}

fun submitCancel(cancel: Cancel) {
    try {
        rustCoreSubmitCancel(cancel.rustCoreCancel)
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}

fun getExchange(pfiDidUri: String, bearerDid: BearerDid, exchangeId: String): Exchange {
    try {
        val rustCoreExchange = rustCoreGetExchange(pfiDidUri, RustCoreBearerDid.fromWeb5(bearerDid), exchangeId)
        return Exchange.fromRustCore(rustCoreExchange)
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}

data class GetExchangeIdsQueryParams (
    val paginationOffset: Long?,
    val paginationLimit: Long?
) {
    internal fun toRustCore(): RustCoreGetExchangeIdsQueryParams {
        return RustCoreGetExchangeIdsQueryParams(paginationOffset, paginationLimit)
    }
}

fun getExchangeIds(pfiDidUri: String, bearerDid: BearerDid, queryParams: GetExchangeIdsQueryParams? = null): List<String> {
    try {
        return rustCoreGetExchangeIds(pfiDidUri, RustCoreBearerDid.fromWeb5(bearerDid), queryParams?.toRustCore())
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}



