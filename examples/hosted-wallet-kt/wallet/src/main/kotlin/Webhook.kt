import tbdex.sdk.messages.Close
import tbdex.sdk.messages.OrderStatus
import tbdex.sdk.messages.Quote
import spark.Spark.port
import spark.Spark.post
import spark.Spark.stop
import tbdex.sdk.http.ReplyToRequestBody

class Webhook {
    var quote: Quote? = null
    val orderStatuses: MutableList<OrderStatus> = mutableListOf()
    var close: Close? = null

    init {
        port(8081)

        post("/pfi-reply-to") { req, res ->
            val tmp = req.body()
            val requestBody = ReplyToRequestBody.fromJsonString(tmp)
            when (val message = requestBody.message) {
                is Quote -> quote = message
                is OrderStatus -> orderStatuses.add(message)
                is Close -> close = message
            }

            res.status(202)
            ""
        }
    }

    fun stopServer() {
        stop()
    }
}