package com.example

import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import spark.Spark.*
import tbdex.sdk.httpclient.CreateExchangeRequestBody
import tbdex.sdk.messages.*
import tbdex.sdk.web5.BearerDid

class ExchangesApi(private val bearerDid: BearerDid, private val offeringsApi: OfferingsApi) {
    // this exists b/c rust core doesn't yet support http server features (namely the ability to mutate the Exchange)
    data class TmpDummyExchange(
        val replyTo: String,
        val rfq: Rfq,
        var quote: Quote? = null,
        var order: Order? = null,
    )

    private var exchanges: MutableMap<String, TmpDummyExchange> = mutableMapOf()

    fun setupCreateExchange() {
        post("/exchanges") { req, res ->
            val requestBodyString = req.body()
            println(requestBodyString)
            val requestBody = CreateExchangeRequestBody(requestBodyString)
            println(requestBody)
            val rfq = requestBody.rfq as Rfq
            println("rfq: $rfq")

            requestBody.rfq.verifyOfferingRequirements(this.offeringsApi.offering)

            exchanges[requestBody.rfq.metadata.exchangeId] = TmpDummyExchange(requestBody.replyTo!!, requestBody.rfq)

            res.status(202)

            // use replyTo to respond with the Quote
            val client = OkHttpClient()
            val mediaType = "application/json; charset=utf-8".toMediaTypeOrNull()
            val quote = Quote(
                bearerDid = this.bearerDid,
                to = requestBody.rfq.metadata.from,
                from = this.bearerDid.did.uri,
                exchangeId = requestBody.rfq.metadata.exchangeId,
                data = QuoteData(
                    expiresAt = "2024-08-02T04:26:08.239Z",
                    payin = QuoteDetails(
                        currencyCode = "BTC",
                        amount = "1000.00",
                        fee = null,
                        paymentInstructions = null
                    ),
                    payout = QuoteDetails(
                        currencyCode = "KES",
                        amount = "123456789.00",
                        fee = null,
                        paymentInstructions = null
                    )
                ),
                "1.0",
                null
            )
            val quoteRequestBody = quote.toJson().toRequestBody(mediaType)

            val request = Request.Builder()
                .url(requestBody.replyTo!!)
                .post(quoteRequestBody)
                .build()

            client.newCall(request).execute().use { response ->
                println(response.code)
            }
        }
    }

    fun setupSubmitOrder() {
        // NOTE: this example doesn't support Close messages from the client
        put("/exchanges/:id") { req, res ->
            val order = Order(req.body())
            println(order)
            exchanges[order.metadata.exchangeId]?.order = order

            res.status(202)

            // use replyTo to respond with the order statuses and close
            val client = OkHttpClient()
            val mediaType = "application/json; charset=utf-8".toMediaTypeOrNull()

            val orderStatusPending = OrderStatus(
                bearerDid = this.bearerDid,
                to = order.metadata.from,
                from = this.bearerDid.did.uri,
                exchangeId = order.metadata.exchangeId,
                data = OrderStatusData("PENDING"),
                "1.0"
            )
            val orderStatusPendingRequestBody = orderStatusPending.toJson().toRequestBody(mediaType)
            val request1 = Request.Builder()
                .url(exchanges[order.metadata.exchangeId]?.replyTo!!)
                .post(orderStatusPendingRequestBody)
                .build()

            client.newCall(request1).execute().use { response ->
                println(response.code)
            }

            val orderStatusCompleted = OrderStatus(
                bearerDid = this.bearerDid,
                to = order.metadata.from,
                from = this.bearerDid.did.uri,
                exchangeId = order.metadata.exchangeId,
                data = OrderStatusData("COMPLETED"),
                "1.0"
            )
            val orderStatusCompletedRequestBody = orderStatusCompleted.toJson().toRequestBody(mediaType)
            val request2 = Request.Builder()
                .url(exchanges[order.metadata.exchangeId]?.replyTo!!)
                .post(orderStatusCompletedRequestBody)
                .build()

            client.newCall(request2).execute().use { response ->
                println(response.code)
            }

            val close = Close(
                bearerDid = this.bearerDid,
                to = order.metadata.from,
                from = this.bearerDid.did.uri,
                exchangeId = order.metadata.exchangeId,
                data = CloseData(
                    reason = null,
                    success = true
                ),
                "1.0"
            )
            val closeRequestBody = close.toJson().toRequestBody(mediaType)
            val request3 = Request.Builder()
                .url(exchanges[order.metadata.exchangeId]?.replyTo!!)
                .post(closeRequestBody)
                .build()

            client.newCall(request3).execute().use { response ->
                println(response.code)
            }

            ""
        }
    }
}