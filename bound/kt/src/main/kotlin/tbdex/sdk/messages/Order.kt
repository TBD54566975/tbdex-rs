package tbdex.sdk.messages

import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.InnerBearerDid
import tbdex.sdk.rust.Order as RustCoreOrder
import web5.sdk.dids.BearerDid

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
            to: String,
            from: String,
            exchangeId: String,
            protocol: String? = null,
            externalId: String? = null
        ): Order {
            val rustCoreOrder = RustCoreOrder.create(to, from, exchangeId, protocol, externalId)
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

    fun sign(bearerDid: BearerDid) {
        val innerBearerDid = InnerBearerDid.fromWeb5(bearerDid)
        this.rustCoreOrder.sign(innerBearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreOrder.verify()
    }
}