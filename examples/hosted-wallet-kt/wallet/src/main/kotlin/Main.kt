import tbdex.sdk.messages.Rfq
import tbdex.sdk.messages.CreateRfqData
import tbdex.sdk.messages.CreateSelectedPayinMethod
import tbdex.sdk.messages.CreateSelectedPayoutMethod
import tbdex.sdk.messages.Order
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
import java.io.File
import java.util.*

const val REPLY_TO = "http://localhost:8081/pfi-reply-to"

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
        replyTo = REPLY_TO
    )
    println("Created exchange ${rfq.metadata.exchangeId}\n")

    // 3. wait for Quote to come into webhook
    println("3. Waiting for Quote...")
    while (webhook.quote == null) {
        Thread.sleep(3000)
    }

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
    while (webhook.orderStatuses.size < 2) {
        Thread.sleep(3000)
    }

    // 6. wait for Close to come into webhook
    println("6. Waiting for Close...")
    while (webhook.close == null) {
        Thread.sleep(3000)
    }

    println("Exchange completed successfully!")
    webhook.stopServer()
}
