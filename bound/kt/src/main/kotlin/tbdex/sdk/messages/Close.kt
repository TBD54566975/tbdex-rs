package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.Close as RustCoreClose
import tbdex.sdk.rust.CloseDataData as RustCoreCloseData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents the data for a Close message in the tbDEX protocol.
 *
 * @property reason The reason why the exchange is being closed.
 * @property success Indicates whether the exchange was successfully completed.
 */
data class CloseData (
    val reason: String?,
    val success: Boolean?
) {
    companion object {
        /**
         * Converts the RustCore close data into a Kotlin `CloseData`.
         *
         * @param rustCore The RustCore representation of close data.
         * @return The Kotlin `CloseData`.
         */
        internal fun fromRustCore(rustCore: RustCoreCloseData): CloseData {
            return CloseData(rustCore.reason, rustCore.success)
        }
    }

    /**
     * Converts the Kotlin `CloseData` into the RustCore equivalent.
     *
     * @return The RustCore representation of close data.
     */
    internal fun toRustCore(): RustCoreCloseData {
        return RustCoreCloseData(reason, success)
    }
}

/**
 * Represents a Close message in the tbDEX protocol.
 *
 * A Close message is sent by a PFI to Alice to signal the termination of an exchange,
 * either because the exchange was successfully completed or because it cannot be fulfilled.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property data The data part of the Close message, including the reason for closure and success status.
 * @property signature The signature verifying the authenticity and integrity of the Close message.
 * @property rustCoreClose The underlying RustCore representation of the Close message.
 */
data class Close private constructor(
    val metadata: MessageMetadata,
    val data: CloseData,
    val signature: String,
    internal val rustCoreClose: RustCoreClose
): Message, ReplyToMessage {
    companion object {
        /**
         * Creates a new Close message.
         *
         * @param to The DID of the recipient (Alice).
         * @param from The DID of the sender (the PFI).
         * @param exchangeId The exchange ID shared between Alice and the PFI.
         * @param data The data containing the reason for closure and success status.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created Close message.
         * @throws TbdexException if the creation process fails.
         */
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

        /**
         * Parses a Close message from a JSON string.
         *
         * @param json The JSON string representing the Close message.
         * @return The deserialized Close message.
         * @throws TbdexException if parsing fails.
         */
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

        /**
         * Converts a RustCore Close message into a Kotlin Close message.
         *
         * @param rustCoreClose The RustCore representation of the Close message.
         * @return The Kotlin Close message.
         */
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

    /**
     * Serializes the Close message to a JSON string.
     *
     * @return The serialized JSON string of the Close message.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreClose.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Close message using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Close message.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreClose.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Close message's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreClose.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}