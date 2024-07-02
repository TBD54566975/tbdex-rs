package com.example

import spark.Spark.get
import spark.Spark.post
import spark.Spark.port
import tbdex.sdk.messages.*
import tbdex.sdk.rust.MessageKind
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
import java.nio.file.Files
import java.nio.file.Paths

// TODO use env vars
const val PFI_DID_URI = "did:dht:ysyokwn6mxnzihgnhkkesjig8cdb3r94eq8abp3a7e935y4s3c4y"
//const val VC = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiU2FuY3Rpb25DcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6ZjBkYWNlZmItNDVlNy00YWEyLTkxNDctMTZmYTBiYzc3ZTVjIiwiaXNzdWVyIjoiZGlkOmRodDpjOGQ5aHVrN25ob200bjd6Z3JtYTZwanljeTY3NHVyYWE0cG9wOXV1dDQxZ2I5d3k4c255IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNi0yNFQxNDoxNTozNVoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImJlZXAiOiJib29wIn19LCJuYmYiOjE3MTkyMzg1MzUsImp0aSI6InVybjp1dWlkOmYwZGFjZWZiLTQ1ZTctNGFhMi05MTQ3LTE2ZmEwYmM3N2U1YyIsImlzcyI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImlhdCI6MTcxOTIzODUzNX0.DvDFIl8BTuHRk7VkB82OhYpX0WzBb3BucvAqfXiS92QCiRokXCgQAsOwbbSODoDaFWbHG0BJmWM-eDPcCoucCw"
const val VC = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJTYW5jdGlvbkNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDp5c3lva3duNm14bnppaGduaGtrZXNqaWc4Y2RiM3I5NGVxOGFicDNhN2U5MzV5NHMzYzR5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNy0wMlQwNDoyNDoxNC4yNzYzMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OjFmczVobnhzZ3R4Z2RyNHd6cWkzOGNuajQ2YjF3aGhuOTRvandvNjZnOGhzYzVidDNmZ3kifX0sImlzcyI6ImRpZDpkaHQ6eXN5b2t3bjZteG56aWhnbmhra2VzamlnOGNkYjNyOTRlcThhYnAzYTdlOTM1eTRzM2M0eSIsImp0aSI6InVybjp2Yzp1dWlkOjRiOTUwOWE4LTQwMjgtNDRkOC05OGE0LWRiODg1MjhmNjY4YyIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsIm5iZiI6MTcxOTg5NDI1NCwiaWF0IjoxNzE5ODk0MjU0fQ.c4ws9jR28jElo_uaW9l5OTL-IPMx4JxWl4De7l_BTk0qNhcFlRtR-U0b9087CUOdpNu25XGZzn-R_EVImRGgCw"
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

    var offeringId = ""
    var exchangeId = ""

    get("/frontend") { _, res ->
        res.type("text/html")
        htmlContentString
    }

    get("/frontend/api/offerings") { _, res ->
        val offerings = tbdex.sdk.httpclient.getOfferings(PFI_DID_URI)
        offeringId = offerings[0].metadata.id
        res.type("application/json")
        offerings.map { it.toJson() }
    }

    post("/frontend/api/submit-rfq") { _, res ->
        try {
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

            exchangeId = rfq.metadata.id

            tbdex.sdk.httpclient.createExchange(rfq, "http://localhost:8081/pfi-reply-to")

            res.type("application/json")
            rfq.toJson()
        } catch (ex: Exception) {
            println(ex.message)
            println(ex)
        }
    }

    var quote: Quote? = null
    val orderStatuses: MutableList<OrderStatus> = mutableListOf()
    var close: Close? = null

    post("/pfi-reply-to") { req, res ->
        try {
            // todo we need parse support

            println("received callback")
            val requestBodyString = req.body()

            try {
                quote = Quote(requestBodyString)
            } catch (ex: Exception) {
                try {
                    orderStatuses.add(OrderStatus(requestBodyString))
                } catch (ex: Exception) {
                    try {
                        close = Close(requestBodyString)
                    } catch (ex: Exception) {
                        throw ex
                    }
                }
            }
        } catch (ex: Exception) {
            println(ex.message)
            println(ex)
        }

        res.status(202)
        ""
    }

    get("/frontend/api/poll-quote") { _, res ->
        if (quote != null) {
            res.type("application/json")
            res.status(200)
            quote!!.toJson()
        } else {
            res.status(204)
            "{}"
        }
    }

    post("/frontend/api/submit-order") { _, res ->
        val order = Order(
            bearerDid,
            PFI_DID_URI,
            bearerDid.did.uri,
            exchangeId,
            "1.0", null
        )

        tbdex.sdk.httpclient.submitOrder(order)

        res.type("application/json")
        order.toJson()
    }

    get("/frontend/api/poll-order-statuses") { _, res ->
        if (orderStatuses.size > 0) {
            res.type("application/json")
            res.status(200)
            orderStatuses.map { it.toJson() }
        } else {
            res.status(204)
            "{}"
        }
    }

    get("/frontend/api/poll-close") { _, res ->
        if (close != null) {
            res.type("application/json")
            res.status(200)
            close!!.toJson()
        } else {
            res.status(204)
            "{}"
        }
    }
}
