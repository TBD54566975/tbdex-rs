package api

import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.OkHttpClient
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Request as OkHttpRequest
import spark.Request
import spark.Response
import spark.Spark.*
import tbdex.sdk.http.*
import tbdex.sdk.httpclient.Exchange
import tbdex.sdk.messages.*
import tbdex.sdk.TbdexException
import web5.sdk.dids.BearerDid

class Exchanges(private val bearerDid: BearerDid, private val offeringsRepository: data.Offerings) {
    init {
        get("/exchanges") { req, res -> getExchangeIds(req, res) }
        get("/exchanges/:id") { req, res -> getExchange(req,res) }

        post("/exchanges") { req, res -> createExchange(req, res) }
        put("/exchanges/:id") { req, res -> updateExchange(req, res) }
    }

    private var exchangesToReplyTo: MutableMap<String, String> = mutableMapOf()
    private var exchangeIdToExchange: MutableMap<String, Exchange> = mutableMapOf()

    private fun getExchange(req: Request, res: Response): String {
        println("GET /exchanges/:id")
        val exchangeId = req.params(":id") ?: throw IllegalArgumentException("Missing exchangeId")

        val exchange = exchangeIdToExchange[exchangeId]

        val messages: List<Message> = listOfNotNull(
            exchange?.rfq,
            exchange?.quote,
            exchange?.order,
            exchange?.orderInstructions,
            exchange?.cancel,
            exchange?.close
        ).plus(exchange?.orderStatuses.orEmpty())

        val responseBody = GetExchangeResponseBody(messages)
        return responseBody.toJsonString()
    }

    private fun getExchangeIds(req: Request, res: Response): String {
        println("GET /exchanges")

        val offset = req.queryParams("page[offset]")?.toIntOrNull() ?: 0
        val limit = req.queryParams("page[limit]")?.toIntOrNull() ?: 10
        val paginatedExchanges = exchangeIdToExchange.keys.toList().drop(offset).take(limit)

        val responseBody = GetExchangesResponseBody(paginatedExchanges)

        res.type("application/json")
        return responseBody.toJsonString()
    }

    private fun createExchange(req: Request, res: Response): String {
        println("POST /exchanges")

        val requestBody = CreateExchangeRequestBody.fromJsonString(req.body())
        val rfq = requestBody.message

        rfq.verify()

        try {
            rfq.verifyOfferingRequirements(this.offeringsRepository.getOffering(rfq.data.offeringId))
        } catch (e: TbdexException) {
            res.status(400)
            val errorResponseBody = ErrorResponseBody("rfq does not satisfy an available offering")
            return errorResponseBody.toJsonString()
        }

        if(requestBody.replyTo != null) {
            this.exchangesToReplyTo[rfq.metadata.exchangeId] = requestBody.replyTo!!
        }
        this.exchangeIdToExchange[rfq.metadata.exchangeId] = Exchange(rfq)

        res.status(202)

        Thread {
            Thread.sleep(500)
            replyWithQuote(rfq.metadata.from, rfq.metadata.exchangeId)
        }.start()

        return ""
    }

    private fun updateExchange(req: Request, res: Response): String {
        println("PUT /exchanges/:id")

        val updateExchangeRequestBody = UpdateExchangeRequestBody.fromJsonString(req.body())
        when (val message = updateExchangeRequestBody.message) {
            is Order -> {
                // simulate order execution
                message.verify()

                Thread {
                    Thread.sleep(500)
                    replyWithOrderInstructions(message.metadata.from, message.metadata.exchangeId)
                    Thread.sleep(500)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId, Status.PAYIN_INITIATED)
                    Thread.sleep(500)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYIN_SETTLED)
                    Thread.sleep(500)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYOUT_INITIATED)
                    Thread.sleep(500)
                    replyWithOrderStatus(message.metadata.from, message.metadata.exchangeId,Status.PAYOUT_SETTLED)
                    Thread.sleep(500)
                    replyWithClose(message.metadata.from, message.metadata.exchangeId)
                }.start()
            }
            is Cancel -> {
                // simulate cancel
                message.verify()

                Thread {
                    Thread.sleep(500)
                    replyWithClose(message.metadata.from, message.metadata.exchangeId, false)
                }.start()
            }
        }

        res.status(202)

        return ""
    }

    private fun replyWithQuote(to: String, exchangeId: String) {
        val quote = Quote.create(
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = QuoteData(
                expiresAt = "2024-08-02T04:26:08.239Z",
                payin = QuoteDetails(
                    currencyCode = "BTC",
                    subtotal = "1000.00",
                    total = "1001.00",
                    fee = null
                ),
                payout = QuoteDetails(
                    currencyCode = "KES",
                    subtotal = "1000.00",
                    total = "1001.00",
                    fee = null
                ),
                payoutUnitsPerPayinUnit = "1.0"
            )
        )

        quote.sign(bearerDid)
        quote.verify()

        println("Replying with quote")

        this.exchangeIdToExchange[exchangeId] = this.exchangeIdToExchange[exchangeId]!!.copy(quote = quote)

        val replyTo = this.exchangesToReplyTo[exchangeId]
        if (replyTo != null) {
            this.replyRequest(replyTo, ReplyToRequestBody(quote))
        }
    }

    private fun replyWithOrderInstructions(to: String, exchangeId: String) {
        val orderInstructions = OrderInstructions.create(
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = OrderInstructionsData(
                payin = PaymentInstruction("http://tbd.website/payin", "payin instruction"),
                payout = PaymentInstruction("http://tbd.website/payout", "payout instruction")
            )
        )

        orderInstructions.sign(bearerDid)
        orderInstructions.verify()

        println("Replying with orderInstructions")

        this.exchangeIdToExchange[exchangeId] = this.exchangeIdToExchange[exchangeId]!!.copy(orderInstructions = orderInstructions)

        val replyTo = this.exchangesToReplyTo[exchangeId]
        if (replyTo != null) {
            this.replyRequest(replyTo, ReplyToRequestBody(orderInstructions))
        }
    }

    private fun replyWithOrderStatus(to: String, exchangeId: String, status: Status) {
        val orderStatus = OrderStatus.create(
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = OrderStatusData(status, null)
        )

        orderStatus.sign(bearerDid)
        orderStatus.verify()

        println("Replying with order status $status")

        this.exchangeIdToExchange[exchangeId] = this.exchangeIdToExchange[exchangeId]!!.let { existingExchange ->
            val updatedOrderStatuses = existingExchange.orderStatuses?.toMutableList() ?: mutableListOf()
            updatedOrderStatuses.add(orderStatus)
            existingExchange.copy(orderStatuses = updatedOrderStatuses)
        }

        val replyTo = this.exchangesToReplyTo[exchangeId]
        if (replyTo != null) {
            this.replyRequest(replyTo, ReplyToRequestBody(orderStatus))
        }
    }

    private fun replyWithClose(to: String, exchangeId: String, success: Boolean? = true) {
        val close = Close.create(
            to = to,
            from = this.bearerDid.did.uri,
            exchangeId = exchangeId,
            data = CloseData(
                reason = null,
                success = success
            )
        )

        close.sign(bearerDid)
        close.verify()

        println("Replying with close")

        this.exchangeIdToExchange[exchangeId] = this.exchangeIdToExchange[exchangeId]!!.copy(close = close)

        val replyTo = this.exchangesToReplyTo[exchangeId]
        if (replyTo != null) {
            this.replyRequest(replyTo, ReplyToRequestBody(close))
        }
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