package tbdex.sdk.http

import tbdex.sdk.messages.*
import tbdex.sdk.rust.MessageKind
import tbdex.sdk.rust.CreateExchangeRequestBody as RustCoreCreateExchangeRequestBody
import tbdex.sdk.rust.GetExchangeResponseBody as RustCoreGetExchangeResponseBody
import tbdex.sdk.rust.GetExchangesResponseBody as RustCoreGetExchangesResponseBody
import tbdex.sdk.rust.UpdateExchangeRequestBody as RustCoreUpdateExchangeRequestBody
import tbdex.sdk.rust.SystemArchitecture

class GetExchangeResponseBody private constructor(
    val data: List<Message>,
    private val rustCoreGetExchangeResponseBody: RustCoreGetExchangeResponseBody
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromJsonString(json: String): GetExchangeResponseBody {
            val rustCoreGetExchangeResponseBody = RustCoreGetExchangeResponseBody.fromJsonString(json)
            val data = rustCoreGetExchangeResponseBody.getData().data.map {
                when (it.kind) {
                    MessageKind.RFQ -> Rfq(it.jsonSerialized)
                    MessageKind.QUOTE -> Quote(it.jsonSerialized)
                    MessageKind.ORDER -> Order(it.jsonSerialized)
                    MessageKind.CANCEL -> Cancel(it.jsonSerialized)
                    MessageKind.ORDER_STATUS -> OrderStatus(it.jsonSerialized)
                    MessageKind.CLOSE -> Close(it.jsonSerialized)
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
    private val rustCoreGetExchangesResponseBody: RustCoreGetExchangesResponseBody
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

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
    private val rustCoreCreateExchangeRequestBody: RustCoreCreateExchangeRequestBody
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromJsonString(json: String): CreateExchangeRequestBody {
            val rustCoreCreateExchangeRequestBody = RustCoreCreateExchangeRequestBody.fromJsonString(json)
            val data = rustCoreCreateExchangeRequestBody.getData()
            return CreateExchangeRequestBody(
                Rfq(data.message),
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
    private val rustCoreUpdateExchangeRequestBody: RustCoreUpdateExchangeRequestBody
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromJsonString(json: String): UpdateExchangeRequestBody {
            val rustCoreUpdateExchangeRequestBody = RustCoreUpdateExchangeRequestBody.fromJsonString(json)
            val data = rustCoreUpdateExchangeRequestBody.getData()

            val message = when (data.kind) {
                MessageKind.ORDER -> Order(data.jsonSerializedMessage)
                MessageKind.CANCEL -> Cancel(data.jsonSerializedMessage)
                else -> throw Exception("Unsupported message kind ${data.kind}")
            }

            return UpdateExchangeRequestBody(message, rustCoreUpdateExchangeRequestBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreUpdateExchangeRequestBody.toJsonString()
    }
}