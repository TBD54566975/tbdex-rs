package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.Order as RustCoreOrder
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class Order private constructor(
    val metadata: MessageMetadata,
    val signature: String,
    internal val rustCoreOrder: RustCoreOrder
): Message, WalletUpdateMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            protocol: String? = null,
            externalId: String? = null
        ): Order {
            try {
                val rustCoreOrder = RustCoreOrder.create(to, from, exchangeId, protocol, externalId)
                val rustCoreData = rustCoreOrder.getData()
                return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        fun fromJsonString(json: String): Order {
            try {
                val rustCoreOrder = RustCoreOrder.fromJsonString(json)
                val rustCoreData = rustCoreOrder.getData()
                return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreOrder(rustCoreOrder: RustCoreOrder): Order {
            val rustCoreData = rustCoreOrder.getData()
            return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreOrder.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOrder.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreOrder.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}