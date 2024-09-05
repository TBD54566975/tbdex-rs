package tbdex.sdk.http

import tbdex.sdk.messages.*
import tbdex.sdk.rust.GetExchangeResponseBodyDataSerializedMessage as RustCoreGetExchangeResponseBodyDataSerializedMessage
import tbdex.sdk.rust.MessageKind as RustCoreMessageKind
import tbdex.sdk.rust.CreateExchangeRequestBody as RustCoreCreateExchangeRequestBody
import tbdex.sdk.rust.GetExchangeResponseBody as RustCoreGetExchangeResponseBody
import tbdex.sdk.rust.GetExchangeResponseBodyData as RustCoreGetExchangeResponseBodyData
import tbdex.sdk.rust.GetExchangesResponseBody as RustCoreGetExchangesResponseBody
import tbdex.sdk.rust.UpdateExchangeRequestBody as RustCoreUpdateExchangeRequestBody
import tbdex.sdk.rust.ReplyToRequestBody as RustCoreReplyToRequestBody

data class GetExchangeResponseBody private constructor(
    val data: List<Message>,
    internal val rustCoreGetExchangeResponseBody: RustCoreGetExchangeResponseBody
) {
    constructor(data: List<Message>) : this(
        data,
        RustCoreGetExchangeResponseBody(
            RustCoreGetExchangeResponseBodyData(
                data.map {
                    val (kind, jsonSerialized) = when (it) {
                        is Rfq -> Pair(RustCoreMessageKind.RFQ, it.toJsonString())
                        is Quote -> Pair(RustCoreMessageKind.QUOTE, it.toJsonString())
                        is Order -> Pair(RustCoreMessageKind.ORDER, it.toJsonString())
                        is OrderInstructions -> Pair(RustCoreMessageKind.ORDER_INSTRUCTIONS, it.toJsonString())
                        is Cancel -> Pair(RustCoreMessageKind.CANCEL, it.toJsonString())
                        is OrderStatus -> Pair(RustCoreMessageKind.ORDER_STATUS, it.toJsonString())
                        is Close -> Pair(RustCoreMessageKind.CLOSE, it.toJsonString())
                        else -> throw Exception("unknown type $it")
                    }

                    RustCoreGetExchangeResponseBodyDataSerializedMessage(kind, jsonSerialized)
                }
            )
        )
    )

    companion object {
        fun fromJsonString(json: String): GetExchangeResponseBody {
            val rustCoreGetExchangeResponseBody = RustCoreGetExchangeResponseBody.fromJsonString(json)
            val data = rustCoreGetExchangeResponseBody.getData().data.map {
                when (it.kind) {
                    RustCoreMessageKind.RFQ -> Rfq.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.QUOTE -> Quote.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.ORDER -> Order.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.ORDER_INSTRUCTIONS -> Order.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.CANCEL -> Cancel.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.ORDER_STATUS -> OrderStatus.fromJsonString(it.jsonSerialized)
                    RustCoreMessageKind.CLOSE -> Close.fromJsonString(it.jsonSerialized)
                }
            }

            return GetExchangeResponseBody(data, rustCoreGetExchangeResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetExchangeResponseBody.toJsonString()
    }
}

data class GetExchangesResponseBody private constructor(
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

data class CreateExchangeRequestBody private constructor(
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

data class UpdateExchangeRequestBody private constructor(
    val message: WalletUpdateMessage,
    internal val rustCoreUpdateExchangeRequestBody: RustCoreUpdateExchangeRequestBody
) {
    constructor(message: WalletUpdateMessage) : this(
        message,
        when (message) {
            is Order -> RustCoreUpdateExchangeRequestBody(RustCoreMessageKind.ORDER_STATUS, message.toJsonString())
            is Cancel -> RustCoreUpdateExchangeRequestBody(RustCoreMessageKind.CANCEL, message.toJsonString())
            else -> throw Exception("unknown type")
        }
    )

    companion object {
        fun fromJsonString(json: String): UpdateExchangeRequestBody {
            val rustCoreUpdateExchangeRequestBody = RustCoreUpdateExchangeRequestBody.fromJsonString(json)
            val data = rustCoreUpdateExchangeRequestBody.getData()

            val message = when (data.kind) {
                RustCoreMessageKind.ORDER -> Order.fromJsonString(data.jsonSerializedMessage)
                RustCoreMessageKind.CANCEL -> Cancel.fromJsonString(data.jsonSerializedMessage)
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

data class ReplyToRequestBody private constructor(
    val message: ReplyToMessage,
    internal val rustCoreReplyToRequestBody: RustCoreReplyToRequestBody
) {
    constructor(message: ReplyToMessage) : this(
        message,
        when (message) {
            is Quote -> RustCoreReplyToRequestBody(RustCoreMessageKind.QUOTE, message.toJsonString())
            is OrderInstructions -> RustCoreReplyToRequestBody(RustCoreMessageKind.ORDER_INSTRUCTIONS, message.toJsonString())
            is OrderStatus -> RustCoreReplyToRequestBody(RustCoreMessageKind.ORDER_STATUS, message.toJsonString())
            is Close -> RustCoreReplyToRequestBody(RustCoreMessageKind.CLOSE, message.toJsonString())
            else -> throw Exception("unknown type")
        }
    )

    companion object {
        fun fromJsonString(json: String): ReplyToRequestBody {
            val rustCoreReplyToRequestBody = RustCoreReplyToRequestBody.fromJsonString(json)
            val data = rustCoreReplyToRequestBody.getData()

            val message: ReplyToMessage = when (data.kind) {
                RustCoreMessageKind.QUOTE -> Quote.fromJsonString(data.jsonSerializedMessage)
                RustCoreMessageKind.ORDER_INSTRUCTIONS -> OrderInstructions.fromJsonString(data.jsonSerializedMessage)
                RustCoreMessageKind.ORDER_STATUS -> OrderStatus.fromJsonString(data.jsonSerializedMessage)
                RustCoreMessageKind.CLOSE -> Close.fromJsonString(data.jsonSerializedMessage)
                else -> throw Exception("Unsupported message kind ${data.kind}")
            }

            return ReplyToRequestBody(message, rustCoreReplyToRequestBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreReplyToRequestBody.toJsonString()
    }
}
