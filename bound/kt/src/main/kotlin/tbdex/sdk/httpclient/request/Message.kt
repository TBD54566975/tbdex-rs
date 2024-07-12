package tbdex.sdk.httpclient.request

import tbdex.sdk.messages.Rfq
import tbdex.sdk.messages.Quote
import tbdex.sdk.messages.Order
import tbdex.sdk.messages.OrderStatus
//import tbdex.sdk.messages.Cancel
import tbdex.sdk.messages.Close
import tbdex.sdk.rust.MessageKind

class Message private constructor(
    private val rfq: Rfq?,
    private val quote: Quote?,
    private val order: Order?,
    private val orderStatus: OrderStatus?,
    private val close: Close?,
//    private val cancel: Cancel?
) {
    companion object {
        fun fromJsonString(kind: MessageKind, json: String): Message {
            return when (kind) {
                MessageKind.RFQ -> Message(Rfq(json), null, null, null, null)
                MessageKind.QUOTE -> Message(null, Quote(json), null, null, null)
                MessageKind.ORDER -> Message(null, null, Order(json), null, null)
                MessageKind.CANCEL -> TODO()
                MessageKind.ORDER_STATUS -> Message(null, null, null, OrderStatus(json), null)
                MessageKind.CLOSE -> Message(null, null, null, null, Close(json))
            }
        }

        fun fromRfq(rfq: Rfq): Message = Message(rfq, null, null, null, null)
        fun fromQuote(quote: Quote): Message = Message(null, quote, null, null, null)
        fun fromOrder(order: Order): Message = Message(null, null, order, null, null)
        fun fromOrderStatus(orderStatus: OrderStatus): Message = Message(null, null, null, orderStatus, null)
        fun fromClose(close: Close): Message = Message(null, null, null, null, close)
    }

    // todo all of the below
    fun asRfq(): Rfq? = this.rfq
    fun asQuote(): Quote? = this.quote
    fun asOrder(): Order? = this.order
    fun asOrderStatus(): OrderStatus? = this.orderStatus
//    fun asCancel(): Cancel? = null
    fun asClose(): Close? = this.close

    fun toJson(): String {
        return this.rfq?.toJson()
            ?: this.quote?.toJson()
            ?: this.order?.toJson()
//            ?: this.cancel?.toJson()
            ?: this.orderStatus?.toJson()
            ?: this.close?.toJson()
            ?: throw Error("message cannot be empty")
    }
}