import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

fun runHappyPathPollingFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid
) {
    throw NotImplementedError("Polling flow is not implemented yet!")
//    println("1. Fetching offerings...")
//    val offerings = tbdex.sdk.httpclient.getOfferings(pfiDidUri)
//    val offeringId = offerings[0].metadata.id
//    println("Successfully fetched $offeringId\n")
//
//    println("2. Creating exchange...")
//    val rfq = Rfq.create(
//        pfiDidUri,
//        bearerDid.did.uri,
//        CreateRfqData(
//            offeringId = offeringId,
//            payin = CreateSelectedPayinMethod(
//                "USD_LEDGER",
//                null,
//                "101"
//            ),
//            payout = CreateSelectedPayoutMethod(
//                "MOMO_MPESA",
//                mapOf(
//                    "phoneNumber" to "867-5309",
//                    "reason" to "cause"
//                )
//            ),
//            claims = listOf(verifiableCredential)
//        )
//    )
//
//    rfq.sign(bearerDid)
//    rfq.verify()
//
//    tbdex.sdk.httpclient.createExchange(rfq = rfq)
//    println("Created exchange ${rfq.metadata.exchangeId}\n")
//
//    println("3. Waiting for Quote...")
//    var quote: Quote? = null
//    while (quote == null) {
//        Thread.sleep(500)
//        quote = tbdex.sdk.httpclient.getQuote(rfq.metadata.exchangeId)
//    }
//    println("Quote received ${quote.metadata.id}\n")
//
//    println("4. Submitting order...")
//    val order = Order.create(
//        pfiDidUri,
//        bearerDid.did.uri,
//        rfq.metadata.exchangeId
//    )
//    order.sign(bearerDid)
//    order.verify()
//    tbdex.sdk.httpclient.submitOrder(order = order)
//    println("Order submitted ${order.metadata.id}\n")
//
//    println("5. Waiting for OrderStatuses...")
//    var status: Status? = null
//    while (status != Status.PAYOUT_SETTLED) {
//        Thread.sleep(500)
//        val orderStatus = tbdex.sdk.httpclient.getOrderStatus(rfq.metadata.exchangeId)
//        if (orderStatus != null) {
//            status = orderStatus.data.status
//        }
//    }
//
//    println("\n6. Waiting for Close...")
//    var close: Close? = null
//    while (close == null) {
//        Thread.sleep(500)
//        close = tbdex.sdk.httpclient.getClose(rfq.metadata.exchangeId)
//    }
//    println("Close received ${close.metadata.id} ${close.data.success}\n")
//
//    println("Exchange completed successfully!")
}
