import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

fun runHappyPathPollingFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid
) {
    println("\n ~Running Happy Path Polling Flow~ \n")

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

    tbdex.sdk.httpclient.createExchange(rfq = rfq)
    println("Created exchange ${rfq.metadata.exchangeId}\n")

    val exchangeId = rfq.metadata.exchangeId

    println("3. Waiting for Quote...")
    var quote: Quote? = null
    while (quote == null) {
        Thread.sleep(500)
        val exchange = tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId)
        if(exchange.quote != null) {
            quote = exchange.quote!!
        }
    }

    println("Received quote ${quote.metadata.id}\n")

    println("4. Submitting order...")
    val order = Order.create(
        pfiDidUri,
        bearerDid.did.uri,
        quote.metadata.exchangeId
    )

    order.sign(bearerDid)
    order.verify()

    tbdex.sdk.httpclient.submitOrder(order = order)
    println("Order submitted ${order.metadata.id}\n")

    println("5. Waiting for order instructions...")
    var orderInstructions: OrderInstructions? = null
    while (orderInstructions == null) {
        Thread.sleep(500)
        val exchange = tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId)
        if(exchange.orderInstructions != null) {
            orderInstructions = exchange.orderInstructions
            println("Received order instructions: ${orderInstructions!!.metadata.id}\n")
        }
    }

    println("6. Waiting for order status: PAYOUT_SETTLED...")
    var orderStatuses: List<OrderStatus>? = null
    while (orderStatuses == null) {
        Thread.sleep(500)
        val exchange = tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId)
        if(exchange.orderStatuses != null) {
            for(os in exchange.orderStatuses!!) {
                if(os.data.status.toString() == "PAYOUT_SETTLED") {
                    orderStatuses = exchange.orderStatuses;
                }
            }
        }
    }

    for(orderStatus in orderStatuses!!) {
        println("Received order status ${orderStatus.metadata.id} ${orderStatus.data.status}\n")
    }

    println("7. Waiting for Close...")
    var close: Close? = null
    while (close == null) {
        Thread.sleep(500)
        val exchange = tbdex.sdk.httpclient.getExchange(pfiDidUri, bearerDid, exchangeId)
        if(exchange.close != null) {
            close = exchange.close!!
        }
    }

    println("Close received ${close!!.metadata.id} ${close!!.data.success}\n")
    println("Exchange completed successfully!")
}