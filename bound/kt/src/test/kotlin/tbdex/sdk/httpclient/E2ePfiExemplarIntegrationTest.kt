package tbdex.sdk.httpclient

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.fail
import org.junit.jupiter.api.Test
import tbdex.sdk.messages.*
import tbdex.sdk.rust.RustCoreException
import tbdex.sdk.rust.Web5RustCoreException
import tbdex.sdk.web5.*

class E2ePfiExemplarIntegrationTest {
    @Test
    fun `can make the happy path`() {
        try {
            val pfiDidUri = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy"

            val didUri = "did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy"
            val keyManager = InMemoryKeyManager(listOf())
            val publicJwk = keyManager.importPrivateJwk(Jwk(
                "EdDSA",
                "OKP",
                "Ed25519",
                "jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04",
                "kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw",
                null
            ))
            val bearerDid = BearerDid(didUri, keyManager)

            // get offerings
            val offerings = getOfferings(pfiDidUri)
            println("Successfully retrieved offerings")
            assertNotEquals(0, offerings.size)

            // get balance
            // TODO pfi-exemplar currently returning invalid balances (missing signature)
//            val balances = getBalances(pfiDidUri, bearerDid)
//            println("Successfully retrieved balances")
//            assertNotEquals(0, balances.size)

            // create exchange
            val rfq = Rfq(
                bearerDid,
                pfiDidUri,
                bearerDid.did.uri,
                CreateRfqData(
                    offeringId = offerings[0].metadata.id,
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
                    claims = listOf("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiU2FuY3Rpb25DcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6ZjBkYWNlZmItNDVlNy00YWEyLTkxNDctMTZmYTBiYzc3ZTVjIiwiaXNzdWVyIjoiZGlkOmRodDpjOGQ5aHVrN25ob200bjd6Z3JtYTZwanljeTY3NHVyYWE0cG9wOXV1dDQxZ2I5d3k4c255IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNi0yNFQxNDoxNTozNVoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImJlZXAiOiJib29wIn19LCJuYmYiOjE3MTkyMzg1MzUsImp0aSI6InVybjp1dWlkOmYwZGFjZWZiLTQ1ZTctNGFhMi05MTQ3LTE2ZmEwYmM3N2U1YyIsImlzcyI6ImRpZDpkaHQ6YzhkOWh1azduaG9tNG43emdybWE2cGp5Y3k2NzR1cmFhNHBvcDl1dXQ0MWdiOXd5OHNueSIsInN1YiI6ImRpZDpkaHQ6MWZzNWhueHNndHhnZHI0d3pxaTM4Y25qNDZiMXdoaG45NG9qd282Nmc4aHNjNWJ0M2ZneSIsImlhdCI6MTcxOTIzODUzNX0.DvDFIl8BTuHRk7VkB82OhYpX0WzBb3BucvAqfXiS92QCiRokXCgQAsOwbbSODoDaFWbHG0BJmWM-eDPcCoucCw")
                ),
                "1.0", null
            )
            createExchange(rfq, null)
            println("Successfully created exchange")

            // get quote
            var exchange = getExchange(pfiDidUri, bearerDid, rfq.metadata.exchangeId)
            val quote = exchange.quote ?: throw Exception("Quote should not be null")
            println("Successfully retrieved quote")

            // submit order
            submitOrder(Order(
                bearerDid,
                pfiDidUri,
                bearerDid.did.uri,
                quote.metadata.exchangeId,
                "1.0", null
            ))
            println("Successfully submitted order")

            // get order status and close
            var count = 0
            while (exchange.close == null) {
                Thread.sleep(5000)
                exchange = getExchange(pfiDidUri, bearerDid, quote.metadata.exchangeId)
                count += 1
                if (count >= 3) {
                    throw Exception("Tried 3 times to fetch order status and close and failed")
                }
            }

            println("Exchange completed successfully!")
        } catch (ex: RustCoreException) {
            fail("RustCoreException caught || ${ex.errorType()} || ${ex.variant()} || ${ex.message()}")
        } catch (ex: Web5RustCoreException) {
            fail("Web5RustCoreException caught || ${ex.errorType()} || ${ex.variant()} || ${ex.message()}")
        }
    }
}