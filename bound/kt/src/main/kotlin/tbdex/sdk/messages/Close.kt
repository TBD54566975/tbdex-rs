package tbdex.sdk.messages

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.Close as RustCoreClose
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.CloseDataData as RustCoreCloseData

typealias CloseData = RustCoreCloseData

class Close: Message {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val metadata: MessageMetadata
    val data: CloseData
    val signature: String

    val rustCoreClose: RustCoreClose

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        exchangeId: String,
        data: CloseData,
        protocol: String,
        externalId: String? = null
    ) {
        this.rustCoreClose = RustCoreClose(bearerDid.rustCoreBearerDid, to, from, exchangeId, data, protocol, externalId)

        this.metadata = rustCoreClose.getData().metadata
        this.data = rustCoreClose.getData().data
        this.signature = rustCoreClose.getData().signature
    }

    constructor(json: String) {
        this.rustCoreClose = RustCoreClose.fromJsonString(json)

        this.metadata = rustCoreClose.getData().metadata
        this.data = rustCoreClose.getData().data
        this.signature = rustCoreClose.getData().signature
    }

    constructor(rustCoreClose: RustCoreClose) {
        this.rustCoreClose = rustCoreClose

        this.metadata = rustCoreClose.getData().metadata
        this.data = rustCoreClose.getData().data
        this.signature = rustCoreClose.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreClose.toJson()
    }
}