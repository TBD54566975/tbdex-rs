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
            // todo we need parse support

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

            res.status(202)
            ""
        }
    }

    fun stopServer() {
        stop()
    }
}