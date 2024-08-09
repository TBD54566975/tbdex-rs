package tbdex.sdk.messages

import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.Cancel as RustCoreCancel
import tbdex.sdk.rust.CancelDataData as RustCoreCancelData
import web5.sdk.dids.BearerDid

typealias CancelData = RustCoreCancelData

class Cancel private constructor(
    val metadata: MessageMetadata,
    val data: RustCoreCancelData,
    val signature: String,
    internal val rustCoreCancel: RustCoreCancel
): Message, WalletUpdateMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: RustCoreCancelData,
            protocol: String? = null,
            externalId: String? = null
        ): Cancel {
            val rustCoreCancel = RustCoreCancel.create(to, from, exchangeId, data, protocol, externalId)
            val rustCoreData = rustCoreCancel.getData()
            return Cancel(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreCancel)
        }

        fun fromJsonString(json: String): Cancel {
            val rustCoreCancel = RustCoreCancel.fromJsonString(json)
            val rustCoreData = rustCoreCancel.getData()
            return Cancel(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreCancel)
        }

        internal fun fromRustCoreCancel(rustCoreCancel: RustCoreCancel): Cancel {
            val rustCoreData = rustCoreCancel.getData()
            return Cancel(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreCancel)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreCancel.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreCancel.sign(bearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreCancel.verify()
    }
}