package data

class Exchanges {
    // this is just a simple example app so we don't need to store the entire exchange state
    // we only care about persisting the replyTo URL for each exchange
    private var exchangesToReplyTo: MutableMap<String, String> = mutableMapOf()

    fun addExchange(exchangeId: String, replyTo: String) {
        exchangesToReplyTo[exchangeId] = replyTo
    }

    fun getReplyTo(exchangeId: String): String {
        return exchangesToReplyTo[exchangeId] ?: throw Exception("Exchange $exchangeId not found")
    }
}