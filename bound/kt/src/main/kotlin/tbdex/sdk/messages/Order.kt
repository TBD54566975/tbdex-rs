package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.WalletUpdateMessage
import tbdex.sdk.rust.Order as RustCoreOrder
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents an Order message in the tbDEX protocol.
 *
 * An Order message is sent by Alice to a PFI to execute a transaction based on a previously provided quote.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property signature The signature verifying the authenticity and integrity of the Order message.
 * @property rustCoreOrder The underlying RustCore representation of the Order.
 */
data class Order private constructor(
    val metadata: MessageMetadata,
    val signature: String,
    internal val rustCoreOrder: RustCoreOrder
): Message, WalletUpdateMessage {
    companion object {
        /**
         * Creates a new Order message.
         *
         * @param to The DID of the recipient (the PFI).
         * @param from The DID of the sender (Alice).
         * @param exchangeId The exchange ID shared between Alice and the PFI.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created Order message.
         * @throws TbdexException if the creation process fails.
         */
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            protocol: String? = null,
            externalId: String? = null
        ): Order {
            try {
                val rustCoreOrder = RustCoreOrder.create(to, from, exchangeId, protocol, externalId)
                val rustCoreData = rustCoreOrder.getData()
                return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Parses an Order message from a JSON string.
         *
         * @param json The JSON string representing the Order.
         * @return The deserialized Order message.
         * @throws TbdexException if parsing fails.
         */
        fun fromJsonString(json: String): Order {
            try {
                val rustCoreOrder = RustCoreOrder.fromJsonString(json)
                val rustCoreData = rustCoreOrder.getData()
                return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreOrder(rustCoreOrder: RustCoreOrder): Order {
            val rustCoreData = rustCoreOrder.getData()
            return Order(MessageMetadata.fromRustCore(rustCoreData.metadata), rustCoreData.signature, rustCoreOrder)
        }
    }

    /**
     * Serializes the Order to a JSON string.
     *
     * @return The serialized JSON string of the Order.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreOrder.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Order using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Order.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOrder.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Order's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreOrder.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}