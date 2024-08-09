package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.OrderInstructionsDataData as RustCoreOrderInstructionsData
import web5.sdk.dids.BearerDid

typealias OrderInstructionsData = RustCoreOrderInstructionsData

class OrderInstructions private constructor(
    val metadata: MessageMetadata,
    val data: OrderInstructionsData,
    val signature: String,
    internal val rustCoreOrderInstructions: tbdex.sdk.rust.OrderInstructions
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: OrderInstructionsData,
            protocol: String? = null,
            externalId: String? = null
        ): OrderInstructions {
            val rustCoreOrderInstructions =
                tbdex.sdk.rust.OrderInstructions.create(to, from, exchangeId, data, protocol, externalId)
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderInstructions)
        }

        fun fromJsonString(json: String): OrderInstructions {
            val rustCoreOrderInstructions = tbdex.sdk.rust.OrderInstructions.fromJsonString(json)
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderInstructions)
        }

        internal fun fromRustCoreOrderInstructions(rustCoreOrderInstructions: tbdex.sdk.rust.OrderInstructions): OrderInstructions {
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderInstructions)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreOrderInstructions.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreOrderInstructions.sign(bearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreOrderInstructions.verify()
    }
}