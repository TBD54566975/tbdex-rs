import tbdex.sdk.messages.Close
import tbdex.sdk.messages.OrderStatus
import tbdex.sdk.messages.Quote
import spark.Spark.port
import spark.Spark.post
import spark.Spark.stop

class Webhook {
    var quote: Quote? = null
    val orderStatuses: MutableList<OrderStatus> = mutableListOf()
    var close: Close? = null

    init {
        port(8081)

        post("/pfi-reply-to") { req, res ->
            val body = tbdex.sdk.httpclient.request.Body(req.body())

            body.message.asQuote()?.let {
                quote = it
            } ?: body.message.asOrderStatus()?.let {
                orderStatuses.add(it)
            } ?: body.message.asClose()?.let {
                close = it
            }

            res.status(202)
            ""
        }
    }

    fun stopServer() {
        stop()
    }
}