import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid

// TODO use env vars
const val PFI_DID_URI = "did:dht:ysyokwn6mxnzihgnhkkesjig8cdb3r94eq8abp3a7e935y4s3c4y"
const val VC = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJTYW5jdGlvbkNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDp5c3lva3duNm14bnppaGduaGtrZXNqaWc4Y2RiM3I5NGVxOGFicDNhN2U5MzV5NHMzYzR5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNy0wMlQwNDoyNDoxNC4yNzYzMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OjFmczVobnhzZ3R4Z2RyNHd6cWkzOGNuajQ2YjF3aGhuOTRvandvNjZnOGhzYzVidDNmZ3kifX0sImlzcyI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSIsImp0aSI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsIm5iZiI6MTcxOTg5NDI1NCwiaWF0IjoxNzE5ODk0MjU0fQ.c4ws9jR28jElo_uaW9l5OTL-IPMx4JxWl4De7l_BTk0qNhcFlRtR-U0b9087CUOdpNu25XGZzn-R_EVImRGgCw"
const val PORTABLE_DID_JSON = "{\"uri\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"document\":{\"id\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"verificationMethod\":[{\"id\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\",\"type\":\"JsonWebKey\",\"controller\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"publicKeyJwk\":{\"crv\":\"Ed25519\",\"kty\":\"OKP\",\"x\":\"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw\",\"kid\":\"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI\",\"alg\":\"EdDSA\"}}],\"authentication\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"assertionMethod\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"capabilityDelegation\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"capabilityInvocation\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"]},\"privateKeys\":[{\"crv\":\"Ed25519\",\"d\":\"jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04\",\"kty\":\"OKP\",\"x\":\"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw\",\"kid\":\"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI\",\"alg\":\"EdDSA\"}]}"
const val REPLY_TO = "http://localhost:8081/pfi-reply-to"

fun main() {
    val bearerDid = BearerDid(PortableDid(PORTABLE_DID_JSON))

    // 0. setup replyTo endpoint (for webhook callbacks for Quote, OrderStatus, and Close
    val webhook = Webhook()

    // 1. fetch offering
    println("1. Fetching offerings...")
    val offerings = tbdex.sdk.httpclient.getOfferings(PFI_DID_URI)
    val offeringId = offerings[0].metadata.id
    println("Successfully fetched $offeringId\n")

    // 2. create exchange
    println("2. Creating exchange...")
    val rfq = Rfq(
        bearerDid,
        PFI_DID_URI,
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
            claims = listOf(VC)
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
    println("Quote received to webhook ${webhook.quote!!.metadata.id}\n")

    // 4. submit order
    println("4. Submitting order...")
    val order = Order(
        bearerDid,
        PFI_DID_URI,
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
    println("OrderStatuses received to webhook ${
        webhook.orderStatuses.map { listOf(it.metadata.id, it.data.orderStatus).joinToString(", ") 
    }}\n")

    // 6. wait for Close to come into webhook
    println("6. Waiting for Close...")
    while (webhook.close == null) {
        Thread.sleep(3000)
    }
    println("Close received to webhook ${webhook.close!!.metadata.id} ${webhook.close!!.data.success}")

    println("Exchange completed successfully!")
    webhook.stopServer()
}
