package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.OrderStatusStatus as RustCoreStatus
import tbdex.sdk.rust.OrderStatus as RustCoreOrderStatus
import tbdex.sdk.rust.OrderStatusDataData as RustCoreOrderStatusData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

typealias OrderStatusData = RustCoreOrderStatusData
typealias Status = RustCoreStatus

class OrderStatus private constructor(
    val metadata: MessageMetadata,
    val data: OrderStatusData,
    val signature: String,
    internal val rustCoreOrderStatus: RustCoreOrderStatus
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: OrderStatusData,
            protocol: String? = null,
            externalId: String? = null
        ): OrderStatus {
            val rustCoreOrderStatus = RustCoreOrderStatus.create(to, from, exchangeId, data, protocol, externalId)
            val rustCoreData = rustCoreOrderStatus.getData()
            return OrderStatus(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderStatus)
        }

        fun fromJsonString(json: String): OrderStatus {
            val rustCoreOrderStatus = RustCoreOrderStatus.fromJsonString(json)
            val rustCoreData = rustCoreOrderStatus.getData()
            return OrderStatus(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderStatus)
        }

        internal fun fromRustCoreOrderStatus(rustCoreOrderStatus: RustCoreOrderStatus): OrderStatus {
            val rustCoreData = rustCoreOrderStatus.getData()
            return OrderStatus(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreOrderStatus)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreOrderStatus.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreOrderStatus.sign(RustCoreBearerDid.fromWeb5(bearerDid))
    }

    fun verify() {
        this.rustCoreOrderStatus.verify()
    }
}