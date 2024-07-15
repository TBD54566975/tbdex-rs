package tbdex.sdk.messages

import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.Cancel as RustCoreCancel
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.CancelDataData as RustCoreCancelData

typealias CancelData = RustCoreCancelData

class Cancel: Message, WalletUpdateMessage {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val metadata: MessageMetadata
    val data: RustCoreCancelData
    val signature: String

    val rustCoreCancel: RustCoreCancel

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        exchangeId: String,
        data: RustCoreCancelData,
        protocol: String,
        externalId: String? = null
    ) {
        this.rustCoreCancel = RustCoreCancel(bearerDid.rustCoreBearerDid, to, from, exchangeId, data, protocol, externalId)

        this.metadata = rustCoreCancel.getData().metadata
        this.data = rustCoreCancel.getData().data
        this.signature = rustCoreCancel.getData().signature
    }

    constructor(json: String) {
        this.rustCoreCancel = RustCoreCancel.fromJsonString(json)

        this.metadata = rustCoreCancel.getData().metadata
        this.data = rustCoreCancel.getData().data
        this.signature = rustCoreCancel.getData().signature
    }

    constructor(rustCoreCancel: RustCoreCancel) {
        this.rustCoreCancel = rustCoreCancel

        this.metadata = rustCoreCancel.getData().metadata
        this.data = rustCoreCancel.getData().data
        this.signature = rustCoreCancel.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreCancel.toJson()
    }
}