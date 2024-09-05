package tbdex.sdk.messages

import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.Cancel as RustCoreCancel
import tbdex.sdk.rust.CancelDataData as RustCoreCancelData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class CancelData(val reason: String? = null) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreCancelData): CancelData {
            return CancelData(rustCore.reason)
        }
    }
}

data class Cancel private constructor(
    val metadata: MessageMetadata,
    val data: CancelData,
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
            return Cancel(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                CancelData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreCancel
            )
        }

        fun fromJsonString(json: String): Cancel {
            val rustCoreCancel = RustCoreCancel.fromJsonString(json)
            val rustCoreData = rustCoreCancel.getData()
            return Cancel(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                CancelData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreCancel
            )
        }

        internal fun fromRustCoreCancel(rustCoreCancel: RustCoreCancel): Cancel {
            val rustCoreData = rustCoreCancel.getData()
            return Cancel(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                CancelData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreCancel
            )
        }
    }

    fun toJsonString(): String {
        return this.rustCoreCancel.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreCancel.sign(RustCoreBearerDid.fromWeb5(bearerDid))
    }

    fun verify() {
        this.rustCoreCancel.verify()
    }
}