package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.Close as RustCoreClose
import tbdex.sdk.rust.CloseDataData as RustCoreCloseData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class CloseData (
    val reason: String?,
    val success: Boolean?
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreCloseData): CloseData {
            return CloseData(rustCore.reason, rustCore.success)
        }
    }

    internal fun toRustCore(): RustCoreCloseData {
        return RustCoreCloseData(reason, success)
    }
}

data class Close private constructor(
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
            try {
                val rustCoreClose = RustCoreClose.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
                val rustCoreData = rustCoreClose.getData()
                return Close(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    CloseData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreClose
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        fun fromJsonString(json: String): Close {
            try {
                val rustCoreClose = RustCoreClose.fromJsonString(json)
                val rustCoreData = rustCoreClose.getData()
                return Close(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    CloseData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreClose
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreClose(rustCoreClose: RustCoreClose): Close {
            val rustCoreData = rustCoreClose.getData()
            return Close(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                CloseData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreClose
            )
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreClose.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreClose.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreClose.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}