package tbdex.sdk.messages

import tbdex.sdk.TbdexException
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

    internal fun toRustCore(): RustCoreCancelData {
        return RustCoreCancelData(reason)
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
            data: CancelData,
            protocol: String? = null,
            externalId: String? = null
        ): Cancel {
            try {
                val rustCoreCancel = RustCoreCancel.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
                val rustCoreData = rustCoreCancel.getData()
                return Cancel(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    CancelData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreCancel
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        fun fromJsonString(json: String): Cancel {
            try {
                val rustCoreCancel = RustCoreCancel.fromJsonString(json)
                val rustCoreData = rustCoreCancel.getData()
                return Cancel(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    CancelData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreCancel
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
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
        try {
            return rustCoreCancel.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreCancel.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreCancel.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}