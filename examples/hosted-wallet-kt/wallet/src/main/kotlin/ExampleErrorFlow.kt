import tbdex.sdk.messages.CreateRfqData
import tbdex.sdk.messages.CreateSelectedPayinMethod
import tbdex.sdk.messages.CreateSelectedPayoutMethod
import tbdex.sdk.messages.Rfq
import tbdex.sdk.TbdexException
import web5.sdk.dids.BearerDid

fun runErrorFlow(
    pfiDidUri: String,
    verifiableCredential: String,
    bearerDid: BearerDid
) {
    println("\n ~Running Error Flow~ \n")

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
                "SOMETHING_INVALID", // NOTE: notice here
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

    try {
        // NOTE: in this example, we're showcasing the error case wherein,
        //       the RFQ does not match an available offering
        tbdex.sdk.httpclient.createExchange(rfq = rfq)
    } catch (e: TbdexException) {
        println("Example error response: $e\n")
    }
}