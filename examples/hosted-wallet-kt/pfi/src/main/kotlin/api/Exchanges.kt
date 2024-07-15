package api

import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.OkHttpClient
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Request as OkHttpRequest
import spark.Request
import spark.Response
import spark.Spark.post
import spark.Spark.put
import tbdex.sdk.http.UpdateExchangeRequestBody
import tbdex.sdk.http.CreateExchangeRequestBody
import tbdex.sdk.http.ReplyToRequestBody
import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

class Exchanges(private val bearerDid: BearerDid, private val offeringsRepository: data.Offerings) {
    init {
        post("/exchanges") { req, res -> createExchange(req, res) }
        put("/exchanges/:id") { req, res -> updateExchange(req, res) }
    }

    private var exchangesToReplyTo: MutableMap<String, String> = mutableMapOf()

    private fun createExchange(req: Request, res: Response): String {
        println("POST /exchanges")

        val requestBody = CreateExchangeRequestBody.fromJsonString(req.body())

        val replyTo = requestBody.replyTo ?: throw Exception("replyTo cannot be null for this example")
        val rfq = requestBody.message

        rfq.verifyOfferingRequirements(this.offeringsRepository.getOffering(rfq.data.offeringId))

        this.exchangesToReplyTo[rfq.metadata.exchangeId] = replyTo

        res.status(202)

        Thread {
            Thread.sleep(3000)
            replyWithQuote(rfq.metadata.from, rfq.metadata.exchangeId)
        }.start()

        return ""
    }

    private fun replyWithQuote(to: String, exchangeId: String) {
        val quote = Quote(
            bearerDid = this.bearerDid,
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = QuoteData(
                expiresAt = "2024-08-02T04:26:08.239Z",
                payin = QuoteDetails(
                    currencyCode = "BTC",
                    subtotal = "1000.00",
                    total = "1001.00",
                    fee = null,
                    paymentInstruction = null
                ),
                payout = QuoteDetails(
                    currencyCode = "KES",
                    subtotal = "1000.00",
                    total = "1001.00",
                    fee = null,
                    paymentInstruction = null
                ),
                payoutUnitsPerPayinUnit = "1.0"
            ),
            "1.0",
            null
        )

        val replyTo = this.exchangesToReplyTo[exchangeId] ?: throw Exception("replyTo cannot be null for this example")

        println("Replying with quote")

        this.replyRequest(replyTo, ReplyToRequestBody(quote))
    }

    private fun updateExchange(req: Request, res: Response): String {
        println("PUT /exchanges/:id")

        val updateExchangeRequestBody = UpdateExchangeRequestBody.fromJsonString(req.body())
        when (val message = updateExchangeRequestBody.message) {
            is Order -> {
                // simulate order execution
                Thread {
                    Thread.sleep(1000)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId, Status.PAYIN_INITIATED)
                    Thread.sleep(1000)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYIN_SETTLED)
                    Thread.sleep(1000)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYOUT_INITIATED)
                    Thread.sleep(1000)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYOUT_SETTLED)
                    Thread.sleep(1000)
                    replyWithClose(message.metadata.from, message.metadata.exchangeId)
                }.start()
            }
            is Cancel -> {
                // simulate cancel
                Thread {
                    Thread.sleep(3000)
                    replyWithClose(message.metadata.from, message.metadata.exchangeId, false)
                }.start()
            }
        }

        res.status(202)

        return ""
    }

    private fun replyWithOrderStatus(to: String, exchangeId: String, status: Status) {
        val orderStatus = OrderStatus(
            bearerDid = this.bearerDid,
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = OrderStatusData(status, null),
            "1.0"
        )

        val replyTo = this.exchangesToReplyTo[exchangeId] ?: throw Exception("replyTo cannot be null")

        println("Replying with order status $status")

        this.replyRequest(replyTo, ReplyToRequestBody(orderStatus))
    }

    private fun replyWithClose(to: String, exchangeId: String, success: Boolean? = true) {
        val close = Close(
            bearerDid = this.bearerDid,
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = CloseData(
                reason = null,
                success = success
            ),
            "1.0"
        )

        val replyTo = this.exchangesToReplyTo[exchangeId] ?: throw Exception("replyTo cannot be null")

        println("Replying with close")

        this.replyRequest(replyTo, ReplyToRequestBody(close))
    }

    private fun replyRequest(replyTo: String, body: ReplyToRequestBody) {
        val client = OkHttpClient()
        val mediaType = "application/json; charset=utf-8".toMediaTypeOrNull()
        val requestBody = body.toJsonString().toRequestBody(mediaType)

        val request = OkHttpRequest.Builder()
            .url(replyTo)
            .post(requestBody)
            .build()

        client.newCall(request).execute()
    }
}