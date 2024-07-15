package tbdex.sdk.messages

import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Order as RustCoreOrder

class Order private constructor(
    val metadata: MessageMetadata,
    val signature: String,
    internal val rustCoreOrder: RustCoreOrder
): Message, WalletUpdateMessage {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun create(
            bearerDid: BearerDid,
            to: String,
            from: String,
            exchangeId: String,
            protocol: String,
            externalId: String? = null
        ): Order {
            val rustCoreOrder = RustCoreOrder(bearerDid.rustCoreBearerDid, to, from, exchangeId, protocol, externalId)
            val rustCoreData = rustCoreOrder.getData()
            return Order(rustCoreData.metadata, rustCoreData.signature, rustCoreOrder)
        }

        fun fromJsonString(json: String): Order {
            val rustCoreOrder = RustCoreOrder.fromJsonString(json)
            val rustCoreData = rustCoreOrder.getData()
            return Order(rustCoreData.metadata, rustCoreData.signature, rustCoreOrder)
        }

        internal fun fromRustCoreOrder(rustCoreOrder: RustCoreOrder): Order {
            val rustCoreData = rustCoreOrder.getData()
            return Order(rustCoreData.metadata, rustCoreData.signature, rustCoreOrder)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreOrder.toJsonString()
    }
}