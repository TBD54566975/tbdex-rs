package com.example

import spark.Spark.*
import tbdex.sdk.messages.*

class ExchangesApi(private val offeringsApi: OfferingsApi) {
    // this exists b/c rust core doesn't yet support http server features (namely the ability to mutate the Exchange)
    data class TmpDummyExchange(
        val replyTo: String,
        val rfq: Rfq,
        var quote: Quote? = null,
        var order: Order? = null,
    )

    private var exchanges: MutableMap<String, TmpDummyExchange> = mutableMapOf()

    data class CreateExchangeRequestBody (
        val replyTo: String,
        val rfq: Rfq
    )

    fun setupCreateExchange() {
        post("/exchanges") { req, res ->
            val requestBody = Json.jsonMapper.readValue(req.body(), CreateExchangeRequestBody::class.java)
            println(requestBody)

            requestBody.rfq.verifyOfferingRequirements(this.offeringsApi.offering)

            exchanges[requestBody.rfq.metadata.exchangeId] = TmpDummyExchange(requestBody.replyTo, requestBody.rfq)

            res.status(202)

            // TODO use replyTo to respond with the Quote
        }
    }

    fun setupSubmitOrder() {
        // NOTE: this example doesn't support Close messages from the client
        put("/exchanges/:id") { req, res ->
            val order = Order(req.body())
            println(order)
            exchanges[order.metadata.exchangeId]?.order = order

            res.status(202)

            // TODO use replyTo to send order statuses and close
        }
    }
}