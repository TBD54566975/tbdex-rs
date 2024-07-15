package tbdex.sdk.http

import tbdex.sdk.messages.Cancel
import tbdex.sdk.messages.Order
import tbdex.sdk.rust.MessageKind
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.UpdateExchangeRequestBody as RustCoreUpdateExchangeRequestBody

interface WalletUpdateMessage {}

class UpdateExchangeRequestBody(
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