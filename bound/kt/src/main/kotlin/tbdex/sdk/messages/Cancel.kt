package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.Cancel as RustCoreCancel
import tbdex.sdk.rust.CancelDataData as RustCoreCancelData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents the data for a Cancel message in the tbDEX protocol.
 *
 * @property reason The reason for canceling the exchange. Optional.
 */
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

/**
 * Represents a Cancel message in the tbDEX protocol.
 *
 * A Cancel message is sent by Alice to a PFI to terminate an exchange that has not been completed,
 * typically when Alice decides to back out of the transaction or request a refund.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property data The data part of the Cancel message, which includes the reason for cancellation.
 * @property signature The signature verifying the authenticity and integrity of the Cancel message.
 * @property rustCoreCancel The underlying RustCore representation of the Cancel message.
 */
data class Cancel private constructor(
    val metadata: MessageMetadata,
    val data: CancelData,
    val signature: String,
    internal val rustCoreCancel: RustCoreCancel
): Message, WalletUpdateMessage {
    companion object {
        /**
         * Creates a new Cancel message.
         *
         * @param to The DID of the recipient (the PFI).
         * @param from The DID of the sender (Alice).
         * @param exchangeId The exchange ID shared between Alice and the PFI.
         * @param data The data containing the reason for cancellation.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created Cancel message.
         * @throws TbdexException if the creation process fails.
         */
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

        /**
         * Parses a Cancel message from a JSON string.
         *
         * @param json The JSON string representing the Cancel message.
         * @return The deserialized Cancel message.
         * @throws TbdexException if parsing fails.
         */
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

    /**
     * Serializes the Cancel message to a JSON string.
     *
     * @return The serialized JSON string of the Cancel message.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreCancel.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Cancel message using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Cancel message.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreCancel.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Cancel message's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreCancel.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}