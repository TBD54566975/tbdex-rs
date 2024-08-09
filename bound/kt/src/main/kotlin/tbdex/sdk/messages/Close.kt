package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.Close as RustCoreClose
import tbdex.sdk.rust.CloseDataData as RustCoreCloseData
import web5.sdk.dids.BearerDid

typealias CloseData = RustCoreCloseData

class Close private constructor(
    val metadata: MessageMetadata,
    val data: CloseData,
    val signature: String,
    internal val rustCoreClose: RustCoreClose
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: CloseData,
            protocol: String? = null,
            externalId: String? = null
        ): Close {
            val rustCoreClose = RustCoreClose.create(to, from, exchangeId, data, protocol, externalId)
            val rustCoreData = rustCoreClose.getData()
            return Close(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreClose)
        }

        fun fromJsonString(json: String): Close {
            val rustCoreClose = RustCoreClose.fromJsonString(json)
            val rustCoreData = rustCoreClose.getData()
            return Close(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreClose)
        }

        internal fun fromRustCoreClose(rustCoreClose: RustCoreClose): Close {
            val rustCoreData = rustCoreClose.getData()
            return Close(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreClose)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreClose.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreClose.sign(bearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreClose.verify()
    }
}