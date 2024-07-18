import tbdex.sdk.messages.Close
import tbdex.sdk.messages.OrderStatus
import tbdex.sdk.messages.Quote
import spark.Spark.port
import spark.Spark.post
import spark.Spark.stop
import tbdex.sdk.http.ReplyToRequestBody

class ReplyToWebhook(
    private val onQuoteReceived: (Quote) -> Unit,
    private val onOrderStatusReceived: (OrderStatus) -> Unit,
    private val onCloseReceived: (Close) -> Unit
) {
    init {
        port(8081)

        post("/pfi-reply-to") { req, res ->
            val requestBody = ReplyToRequestBody.fromJsonString(req.body())
            when (val message = requestBody.message) {
                is Quote -> {
                    message.verify()
                    onQuoteReceived(message)
                }
                is OrderStatus -> {
                    message.verify()
                    onOrderStatusReceived(message)
                }
                is Close -> {
                    message.verify()
                    onCloseReceived(message)
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
