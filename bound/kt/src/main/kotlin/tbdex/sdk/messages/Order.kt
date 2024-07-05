package tbdex.sdk.messages

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Order as RustCoreOrder

class Order {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val metadata: MessageMetadata
    val signature: String

    val rustCoreOrder: RustCoreOrder

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        exchangeId: String,
        protocol: String,
        externalId: String? = null
    ) {
        this.rustCoreOrder = RustCoreOrder(bearerDid.rustCoreBearerDid, to, from, exchangeId, protocol, externalId)

        this.metadata = rustCoreOrder.getData().metadata
        this.signature = rustCoreOrder.getData().signature
    }

    constructor(json: String) {
        this.rustCoreOrder = RustCoreOrder.fromJsonString(json)

        this.metadata = rustCoreOrder.getData().metadata
        this.signature = rustCoreOrder.getData().signature
    }

    constructor(rustCoreOrder: RustCoreOrder) {
        this.rustCoreOrder = rustCoreOrder

        this.metadata = this.rustCoreOrder.getData().metadata
        this.signature = this.rustCoreOrder.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreOrder.toJson()
    }
}