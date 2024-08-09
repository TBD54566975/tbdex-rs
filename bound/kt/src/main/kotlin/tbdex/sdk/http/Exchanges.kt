package tbdex.sdk.http

import tbdex.sdk.messages.*
import tbdex.sdk.rust.GetExchangeResponseBodyDataSerializedMessage
import tbdex.sdk.rust.MessageKind
import tbdex.sdk.rust.CreateExchangeRequestBody as RustCoreCreateExchangeRequestBody
import tbdex.sdk.rust.GetExchangeResponseBody as RustCoreGetExchangeResponseBody
import tbdex.sdk.rust.GetExchangeResponseBodyData as RustCoreGetExchangeResponseBodyData
import tbdex.sdk.rust.GetExchangesResponseBody as RustCoreGetExchangesResponseBody
import tbdex.sdk.rust.UpdateExchangeRequestBody as RustCoreUpdateExchangeRequestBody
import tbdex.sdk.rust.ReplyToRequestBody as RustCoreReplyToRequestBody
import tbdex.sdk.rust.SystemArchitecture

class GetExchangeResponseBody private constructor(
    val data: List<Message>,
    internal val rustCoreGetExchangeResponseBody: RustCoreGetExchangeResponseBody
) {
    constructor(data: List<Message>) : this(
        data,
        RustCoreGetExchangeResponseBody(
            RustCoreGetExchangeResponseBodyData(
                data.map {
                    val (kind, jsonSerialized) = when (it) {
                        is Rfq -> Pair(MessageKind.RFQ, it.toJsonString())
                        is Quote -> Pair(MessageKind.QUOTE, it.toJsonString())
                        is Order -> Pair(MessageKind.ORDER, it.toJsonString())
                        is OrderInstructions -> Pair(MessageKind.ORDER_INSTRUCTIONS, it.toJsonString())
                        is Cancel -> Pair(MessageKind.CANCEL, it.toJsonString())
                        is OrderStatus -> Pair(MessageKind.ORDER_STATUS, it.toJsonString())
                        is Close -> Pair(MessageKind.CLOSE, it.toJsonString())
                        else -> throw Exception("unknown type $it")
                    }

                    GetExchangeResponseBodyDataSerializedMessage(kind, jsonSerialized)
                }
            )
        )
    )

    companion object {
        fun fromJsonString(json: String): GetExchangeResponseBody {
            val rustCoreGetExchangeResponseBody = RustCoreGetExchangeResponseBody.fromJsonString(json)
            val data = rustCoreGetExchangeResponseBody.getData().data.map {
                when (it.kind) {
                    MessageKind.RFQ -> Rfq.fromJsonString(it.jsonSerialized)
                    MessageKind.QUOTE -> Quote.fromJsonString(it.jsonSerialized)
                    MessageKind.ORDER -> Order.fromJsonString(it.jsonSerialized)
                    MessageKind.ORDER_INSTRUCTIONS -> Order.fromJsonString(it.jsonSerialized)
                    MessageKind.CANCEL -> Cancel.fromJsonString(it.jsonSerialized)
                    MessageKind.ORDER_STATUS -> OrderStatus.fromJsonString(it.jsonSerialized)
                    MessageKind.CLOSE -> Close.fromJsonString(it.jsonSerialized)
                }
            }

            return GetExchangeResponseBody(data, rustCoreGetExchangeResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetExchangeResponseBody.toJsonString()
    }
}

class GetExchangesResponseBody private constructor(
    val data: List<String>,
    internal val rustCoreGetExchangesResponseBody: RustCoreGetExchangesResponseBody
) {
    constructor(data: List<String>) : this(
        data,
        RustCoreGetExchangesResponseBody(data)
    )

    companion object {
        fun fromJsonString(json: String): GetExchangesResponseBody {
            val rustCoreGetExchangesResponseBody = RustCoreGetExchangesResponseBody.fromJsonString(json)
            return GetExchangesResponseBody(
                rustCoreGetExchangesResponseBody.getData().data,
                rustCoreGetExchangesResponseBody
            )
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetExchangesResponseBody.toJsonString()
    }
}

class CreateExchangeRequestBody private constructor(
    val message: Rfq,
    val replyTo: String? = null,
    internal val rustCoreCreateExchangeRequestBody: RustCoreCreateExchangeRequestBody
) {
    constructor(message: Rfq, replyTo: String? = null) : this(
        message,
        replyTo,
        RustCoreCreateExchangeRequestBody(message.rustCoreRfq, replyTo)
    )

    companion object {
        fun fromJsonString(json: String): CreateExchangeRequestBody {
            val rustCoreCreateExchangeRequestBody = RustCoreCreateExchangeRequestBody.fromJsonString(json)
            val data = rustCoreCreateExchangeRequestBody.getData()
            return CreateExchangeRequestBody(
                Rfq.fromRustCoreRfq(data.message),
                data.replyTo,
                rustCoreCreateExchangeRequestBody
            )
        }
    }

    fun toJsonString(): String {
        return this.rustCoreCreateExchangeRequestBody.toJsonString()
    }
}

interface WalletUpdateMessage {}

class UpdateExchangeRequestBody private constructor(
    val message: WalletUpdateMessage,
    internal val rustCoreUpdateExchangeRequestBody: RustCoreUpdateExchangeRequestBody
) {
    constructor(message: WalletUpdateMessage) : this(
        message,
        when (message) {
            is Order -> RustCoreUpdateExchangeRequestBody(MessageKind.ORDER_STATUS, message.toJsonString())
            is Cancel -> RustCoreUpdateExchangeRequestBody(MessageKind.CANCEL, message.toJsonString())
            else -> throw Exception("unknown type")
        }
    )

    companion object {
        fun fromJsonString(json: String): UpdateExchangeRequestBody {
            val rustCoreUpdateExchangeRequestBody = RustCoreUpdateExchangeRequestBody.fromJsonString(json)
            val data = rustCoreUpdateExchangeRequestBody.getData()

            val message = when (data.kind) {
                MessageKind.ORDER -> Order.fromJsonString(data.jsonSerializedMessage)
                MessageKind.CANCEL -> Cancel.fromJsonString(data.jsonSerializedMessage)
                else -> throw Exception("Unsupported message kind ${data.kind}")
            }

            return UpdateExchangeRequestBody(message, rustCoreUpdateExchangeRequestBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreUpdateExchangeRequestBody.toJsonString()
    }
}

interface ReplyToMessage {}

class ReplyToRequestBody private constructor(
    val message: ReplyToMessage,
    internal val rustCoreReplyToRequestBody: RustCoreReplyToRequestBody
) {
    constructor(message: ReplyToMessage) : this(
        message,
        when (message) {
            is Quote -> RustCoreReplyToRequestBody(MessageKind.QUOTE, message.toJsonString())
            is OrderInstructions -> RustCoreReplyToRequestBody(MessageKind.ORDER_INSTRUCTIONS, message.toJsonString())
            is OrderStatus -> RustCoreReplyToRequestBody(MessageKind.ORDER_STATUS, message.toJsonString())
            is Close -> RustCoreReplyToRequestBody(MessageKind.CLOSE, message.toJsonString())
            else -> throw Exception("unknown type")
        }
    )

    companion object {
        fun fromJsonString(json: String): ReplyToRequestBody {
            val rustCoreReplyToRequestBody = RustCoreReplyToRequestBody.fromJsonString(json)
            val data = rustCoreReplyToRequestBody.getData()

            val message: ReplyToMessage = when (data.kind) {
                MessageKind.QUOTE -> Quote.fromJsonString(data.jsonSerializedMessage)
                MessageKind.ORDER_INSTRUCTIONS -> OrderInstructions.fromJsonString(data.jsonSerializedMessage)
                MessageKind.ORDER_STATUS -> OrderStatus.fromJsonString(data.jsonSerializedMessage)
                MessageKind.CLOSE -> Close.fromJsonString(data.jsonSerializedMessage)
                else -> throw Exception("Unsupported message kind ${data.kind}")
            }

            return ReplyToRequestBody(message, rustCoreReplyToRequestBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreReplyToRequestBody.toJsonString()
    }
}
