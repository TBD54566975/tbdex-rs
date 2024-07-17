import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

fun runHappyPathFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid,
    replyToUrl: String
) {
    val replyToWebhook = ReplyToWebhook()

    println("1. Fetching offerings...")
    val offerings = tbdex.sdk.httpclient.getOfferings(pfiDidUri)
    val offeringId = offerings[0].metadata.id
    println("Successfully fetched $offeringId\n")

    println("2. Creating exchange...")
    val rfq = Rfq.create(
        pfiDidUri,
        bearerDid.did.uri,
        CreateRfqData(
            offeringId = offeringId,
            payin = CreateSelectedPayinMethod(
                "USD_LEDGER",
                null,
                "101"
            ),
            payout = CreateSelectedPayoutMethod(
                "MOMO_MPESA",
                mapOf(
                    "phoneNumber" to "867-5309",
                    "reason" to "cause"
                )
            ),
            claims = listOf(verifiableCredential)
        )
    )

    rfq.sign(bearerDid)
    rfq.verify()

    tbdex.sdk.httpclient.createExchange(
        rfq = rfq,
        replyTo = replyToUrl
    )
    println("Created exchange ${rfq.metadata.exchangeId}\n")

    println("3. Waiting for Quote...")
    while (replyToWebhook.quote == null) {
        Thread.sleep(500)
    }
    println("Quote received to webhook ${replyToWebhook.quote!!.metadata.id}\n")

    println("4. Submitting order...")
    val order = Order.create(
        pfiDidUri,
        bearerDid.did.uri,
        rfq.metadata.exchangeId
    )

    order.sign(bearerDid)
    order.verify()

    tbdex.sdk.httpclient.submitOrder(
        order = order
    )
    println("Order submitted ${order.metadata.id}\n")

    println("5. Waiting for OrderStatuses...")
    var status: Status? = null
    while (status != Status.PAYOUT_SETTLED) {
        Thread.sleep(500)
        status = if (replyToWebhook.orderStatuses.isNotEmpty()) replyToWebhook.orderStatuses.last().data.status else null
    }

    println("\n6. Waiting for Close...")
    while (replyToWebhook.close == null) {
        Thread.sleep(500)
    }
    println("Close received to webhook ${replyToWebhook.close!!.metadata.id} ${replyToWebhook.close!!.data.success}\n")

    println("Exchange completed successfully!")
    replyToWebhook.stopServer()
}
