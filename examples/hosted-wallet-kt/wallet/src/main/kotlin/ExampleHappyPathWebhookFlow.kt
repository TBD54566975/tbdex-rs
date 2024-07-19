import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

private lateinit var webhook: ReplyToWebhook
private var closedReceived = false

fun runHappyPathFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid,
    replyToUrl: String
) {
    println("\n ~Running Happy Path Webhook Flow~ \n")

    webhook = ReplyToWebhook(
        onQuoteReceived = { quote ->
            println("3. Quote received...")
            println("Quote received from webhook: ${quote.metadata.id}\n")
            handleQuoteReceived(quote, pfiDidUri, bearerDid)
        },
        onOrderStatusReceived = { orderStatus ->
            println("Received order status ${orderStatus.metadata.id} ${orderStatus.data.status}\n")
            handleOrderStatusReceived(orderStatus)
        },
        onCloseReceived = { close ->
            println("Close received: ${close.metadata.id} ${close.data.success}\n")
            handleCloseReceived(close)
        }
    )

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

    // Stay active until closed is received
    while(true) {
        Thread.sleep(500);
        if(closedReceived) {
            break;
        }
    }
}

fun handleQuoteReceived(quote: Quote, pfiDidUri: String, bearerDid: BearerDid) {
    println("4. Submitting order...")
    val order = Order.create(
        pfiDidUri,
        bearerDid.did.uri,
        quote.metadata.exchangeId
    )

    order.sign(bearerDid)
    order.verify()

    tbdex.sdk.httpclient.submitOrder(
        order = order
    )
    println("Order submitted ${order.metadata.id}\n")
}

fun handleOrderStatusReceived(orderStatus: OrderStatus) {
    if (orderStatus.data.status == Status.PAYOUT_SETTLED) {
        orderStatus.verify()
        println("Order settled: ${orderStatus.metadata.id}")
    }
}

fun handleCloseReceived(close: Close) {
    close.verify()
    println("Exchange completed successfully!")

    webhook.stopServer()
    closedReceived = true;
}
