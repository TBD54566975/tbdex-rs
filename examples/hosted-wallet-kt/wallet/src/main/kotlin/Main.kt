import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
import java.io.File
import java.util.*

fun readEnv() {
    val envFile = File(Thread.currentThread().contextClassLoader.getResource(".env")?.file ?: throw Error("no .env file found"))
    val properties = Properties().apply {
        envFile.inputStream().use { load(it) }
    }
    properties.forEach { (key, value) ->
        System.setProperty(key.toString(), value.toString())
    }
}

fun main() {
    readEnv()

    val pfiDidUri = System.getProperty("PFI_DID_URI")
    val verifiableCredential = System.getProperty("HOSTED_WALLET_VERIFIABLE_CREDENTIAL")
    val bearerDid = BearerDid(PortableDid(System.getProperty("HOSTED_WALLET_PORTABLE_DID_JSON")))
    val replyToUrl = System.getProperty("REPLY_TO_URL")
    val showcaseCancelFlow = System.getProperty("SHOWCASE_CANCEL_FLOW").toBoolean()

    // 0. setup replyTo endpoint (for webhook callbacks for Quote, OrderStatus, and Close
    val webhook = Webhook()

    // 1. fetch offering
    println("1. Fetching offerings...")
    val offerings = tbdex.sdk.httpclient.getOfferings(pfiDidUri)
    val offeringId = offerings[0].metadata.id
    println("Successfully fetched $offeringId\n")

    // 2. create exchange
    println("2. Creating exchange...")
    val rfq = Rfq(
        bearerDid,
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
        ),
        "1.0", null
    )
    tbdex.sdk.httpclient.createExchange(
        rfq = rfq,
        replyTo = replyToUrl
    )
    println("Created exchange ${rfq.metadata.exchangeId}\n")

    // 3. wait for Quote to come into webhook
    println("3. Waiting for Quote...")
    while (webhook.quote == null) {
        Thread.sleep(3000)
    }
    println("Quote received to webhook ${webhook.quote!!.metadata.id}\n")

    // configure via SHOWCASE_CANCEL_FLOW env var
    if (showcaseCancelFlow) {
        // 4. submit cancel
        println("4. Submitting cancel...")
        val cancel = Cancel(
            bearerDid,
            pfiDidUri,
            bearerDid.did.uri,
            rfq.metadata.exchangeId,
            CancelData("showcasing an example"),
            "1.0", null
        )
        tbdex.sdk.httpclient.submitCancel(cancel)
        println("Cancel submitted ${cancel.metadata.id}")
    } else {
        // 4. submit order
        println("4. Submitting order...")
        val order = Order(
            bearerDid,
            pfiDidUri,
            bearerDid.did.uri,
            rfq.metadata.exchangeId,
            "1.0", null
        )
        tbdex.sdk.httpclient.submitOrder(
            order = order
        )
        println("Order submitted ${order.metadata.id}\n")

        // 5. wait for OrderStatus to come into webhook
        println("5. Waiting for OrderStatuses...")
        var status: Status? = null
        while (status != Status.PAYOUT_SETTLED) {
            Thread.sleep(1500)
            status = if (webhook.orderStatuses.size > 0) webhook.orderStatuses.last().data.status else null
        }
    }

    // 6. wait for Close to come into webhook
    println("\n6. Waiting for Close...")
    while (webhook.close == null) {
        Thread.sleep(3000)
    }
    println("Close received to webhook ${webhook.close!!.metadata.id} ${webhook.close!!.data.success}\n")

    println("Exchange completed successfully!")
    webhook.stopServer()
}
