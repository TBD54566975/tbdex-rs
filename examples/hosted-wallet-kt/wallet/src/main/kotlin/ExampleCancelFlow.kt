import tbdex.sdk.messages.*
import web5.sdk.dids.BearerDid

private lateinit var webhook: ReplyToWebhook
private var closedReceived = false

fun runCancelFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid,
    replyToUrl: String
) {
    println("\n ~Running Cancel Flow~ \n")

    webhook = ReplyToWebhook(
        onQuoteReceived = { quote ->
            println("3. Quote received...")
            println("Quote received: ${quote.metadata.id}\n")
            handleCancelFlowQuoteReceived(quote, pfiDidUri, bearerDid)
        },
        onOrderStatusReceived = { orderStatus ->
            // this example cancel flow submits the cancel prior to order submission, and so
            // no order statuses are expected to be received
            println("Received order status ${orderStatus.metadata.id} ${orderStatus.data.status}\n")
        },
        onOrderInstructionsReceived = { orderInstructions ->
            // this example cancel flow submits the cancel prior to order submission, and so
            // no order instructions are expected to be received
            println("Received order instructions ${orderInstructions.metadata.id}\n")
        },
        onCloseReceived = { close ->
            println("Close received: ${close.metadata.id} ${close.data.success}\n")
            handleCancelFlowCloseReceived(close)
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

fun handleCancelFlowQuoteReceived(quote: Quote, pfiDidUri: String, bearerDid: BearerDid) {
    println("4. Submitting cancel...")
    val cancel = Cancel.create(
        pfiDidUri,
        bearerDid.did.uri,
        quote.metadata.exchangeId,
        CancelData("showcasing an example")
    )
    cancel.sign(bearerDid)
    cancel.verify()
    tbdex.sdk.httpclient.submitCancel(cancel)
    println("Cancel submitted ${cancel.metadata.id}")
}

fun handleCancelFlowCloseReceived(close: Close) {
    close.verify()
    println("Exchange completed successfully!")

    webhook.stopServer()
    closedReceived = true;
}
