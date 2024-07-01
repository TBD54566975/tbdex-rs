package com.example

import com.fasterxml.jackson.databind.ObjectMapper
import spark.Spark.get
import spark.Spark.post
import spark.Spark.port
import tbdex.sdk.messages.CreateRfqData
import tbdex.sdk.messages.CreateSelectedPayinMethod
import tbdex.sdk.messages.CreateSelectedPayoutMethod
import tbdex.sdk.messages.Rfq
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
import java.nio.file.Files
import java.nio.file.Paths

// TODO use env vars
const val PFI_DID_URI = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy"
const val VC = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiU2FuY3Rpb25DcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6ZjBkYWNlZmItNDVlNy00YWEyLTkxNDctMTZmYTBiYzc3ZTVjIiwiaXNzdWVyIjoiZGlkOmRodDpjOGQ5aHVrN25ob200bjd6Z3JtYTZwanljeTY3NHVyYWE0cG9wOXV1dDQxZ2I5d3k4c255IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNi0yNFQxNDoxNTozNVoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImJlZXAiOiJib29wIn19LCJuYmYiOjE3MTkyMzg1MzUsImp0aSI6InVybjp1dWlkOmYwZGFjZWZiLTQ1ZTctNGFhMi05MTQ3LTE2ZmEwYmM3N2U1YyIsImlzcyI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImlhdCI6MTcxOTIzODUzNX0.DvDFIl8BTuHRk7VkB82OhYpX0WzBb3BucvAqfXiS92QCiRokXCgQAsOwbbSODoDaFWbHG0BJmWM-eDPcCoucCw"
const val PORTABLE_DID_JSON = "{\"uri\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"document\":{\"id\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"verificationMethod\":[{\"id\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\",\"type\":\"JsonWebKey\",\"controller\":\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy\",\"publicKeyJwk\":{\"crv\":\"Ed25519\",\"kty\":\"OKP\",\"x\":\"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw\",\"kid\":\"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI\",\"alg\":\"EdDSA\"}}],\"authentication\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"assertionMethod\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"capabilityDelegation\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"],\"capabilityInvocation\":[\"did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0\"]},\"privateKeys\":[{\"crv\":\"Ed25519\",\"d\":\"jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04\",\"kty\":\"OKP\",\"x\":\"kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw\",\"kid\":\"ezoEr4cxqaYa9eOA3YkvCL1kw9yUuCYl3KMKO79sIbI\",\"alg\":\"EdDSA\"}]}"

fun main() {
    port(8081)

    val classLoader = Thread.currentThread().contextClassLoader
    val resource = classLoader.getResource("public/index.html")
        ?: throw IllegalStateException("Could not find 'public/index.html' in the resources")
    val htmlFilePath = Paths.get(resource.toURI())
    val htmlContent = Files.readAllBytes(htmlFilePath)
    val htmlContentString = String(htmlContent)

    val bearerDid = BearerDid(PortableDid(PORTABLE_DID_JSON))

    get("/frontend") { _, res ->
        res.type("text/html")
        htmlContentString
    }

    get("/frontend/api/offerings") { _, res ->
        val offerings = tbdex.sdk.rust.getOfferings(PFI_DID_URI)
        res.type("application/json")
        offerings.map { it.toJson() }
    }

    post("/frontend/api/exchanges") { _, res ->
        val rfq = Rfq(
            bearerDid,
            PFI_DID_URI,
            bearerDid.did.uri,
            CreateRfqData(
                offeringId = "offering_01j159htqbfcar5qn9094fs301",
                payin = CreateSelectedPayinMethod(
                    "USD_LEDGER",
                    null,
                    "101"
                ),
                payout = CreateSelectedPayoutMethod(
                    "MOMO_MPESA",
                    ObjectMapper().readTree("""{
                        "phoneNumber": "867-5309",
                        "reason": "cause"
                    }""")
                ),
                claims = listOf(VC)
            ),
            "1.0", null
        )

        tbdex.sdk.httpclient.createExchange(rfq) // TODO reply to

        res.type("application/json")
        rfq.toJson()
    }
}
