package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.OrderStatusStatus as RustCoreStatus
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.OrderStatus as RustCoreOrderStatus
import tbdex.sdk.rust.OrderStatusDataData as RustCoreOrderStatusData

typealias OrderStatusData = RustCoreOrderStatusData
typealias Status = RustCoreStatus

class OrderStatus: Message, ReplyToMessage {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val metadata: MessageMetadata
    val data: OrderStatusData
    val signature: String

    val rustCoreOrderStatus: RustCoreOrderStatus

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        exchangeId: String,
        data: OrderStatusData,
        protocol: String,
        externalId: String? = null
    ) {
        this.rustCoreOrderStatus = RustCoreOrderStatus(bearerDid.rustCoreBearerDid, to, from, exchangeId, data, protocol, externalId)

        this.metadata = rustCoreOrderStatus.getData().metadata
        this.data = rustCoreOrderStatus.getData().data
        this.signature = rustCoreOrderStatus.getData().signature
    }

    constructor(json: String) {
        this.rustCoreOrderStatus = RustCoreOrderStatus.fromJsonString(json)

        this.metadata = rustCoreOrderStatus.getData().metadata
        this.data = rustCoreOrderStatus.getData().data
        this.signature = rustCoreOrderStatus.getData().signature
    }

    constructor(rustCoreOrderStatus: RustCoreOrderStatus) {
        this.rustCoreOrderStatus = rustCoreOrderStatus

        this.metadata = this.rustCoreOrderStatus.getData().metadata
        this.data = this.rustCoreOrderStatus.getData().data
        this.signature = this.rustCoreOrderStatus.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreOrderStatus.toJsonString()
    }
}