package api

import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.OkHttpClient
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Request as OkHttpRequest
import spark.Request
import spark.Response
import spark.Spark.post
import spark.Spark.put
import tbdex.sdk.httpclient.CreateExchangeRequestBody
import tbdex.sdk.httpclient.SubmitCancelRequestBody
import tbdex.sdk.httpclient.SubmitOrderRequestBody
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

        val requestBody = CreateExchangeRequestBody(req.body())

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

        this.replyRequest(replyTo, quote.toJson())
    }

    private fun updateExchange(req: Request, res: Response): String {
        println("PUT /exchanges/:id")

        var order: Order? = null
        var cancel: Cancel? = null

        // TODO we're going to implement a parser to alleviate this confusing DX
        val reqBodyText = req.body()
        try {
            val submitOrderRequestBody = SubmitOrderRequestBody(reqBodyText)
            order = submitOrderRequestBody.message
        } catch (e: Exception) {
            val submitCancelRequestBody = SubmitCancelRequestBody(reqBodyText)
            cancel = submitCancelRequestBody.message
        }

        if (order != null) {
            Thread {
                Thread.sleep(3000)
                replyWithOrderStatus(order.metadata.from, order.metadata.exchangeId, "PENDING")
                Thread.sleep(3000)
                replyWithOrderStatus(order.metadata.from, order.metadata.exchangeId,"COMPLETED")
                Thread.sleep(3000)
                replyWithClose(order.metadata.from, order.metadata.exchangeId)
            }.start()
        } else if (cancel != null) {
            Thread {
                Thread.sleep(3000)
                replyWithClose(cancel.metadata.from, cancel.metadata.exchangeId, false)
            }.start()
        }

        res.status(202)

        return ""
    }

    private fun replyWithOrderStatus(to: String, exchangeId: String, status: String) {
        val orderStatus = OrderStatus(
            bearerDid = this.bearerDid,
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = OrderStatusData(status),
            "1.0"
        )

        val replyTo = this.exchangesToReplyTo[exchangeId] ?: throw Exception("replyTo cannot be null")

        println("Replying with order status")

        this.replyRequest(replyTo, orderStatus.toJson())
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

        this.replyRequest(replyTo, close.toJson())
    }

    private fun replyRequest(replyTo: String, body: String) {
        val client = OkHttpClient()
        val mediaType = "application/json; charset=utf-8".toMediaTypeOrNull()
        val requestBody = body.toRequestBody(mediaType)

        val request = OkHttpRequest.Builder()
            .url(replyTo)
            .post(requestBody)
            .build()

        client.newCall(request).execute()
    }
}