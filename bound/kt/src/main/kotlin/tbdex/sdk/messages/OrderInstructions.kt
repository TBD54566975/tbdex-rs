package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.PaymentInstructionData as RustCorePaymentInstruction
import tbdex.sdk.rust.OrderInstructionsDataData as RustCoreOrderInstructionsData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents a payment instruction in the tbDEX protocol, providing details on how to pay or be paid.
 *
 * @property link Optional link for Alice to make a payment or receive a payout.
 * @property instruction Optional instructions for Alice on how to pay or be paid.
 */
data class PaymentInstruction (
    val link: String?,
    val instruction: String?
) {
    companion object {
        /**
         * Converts the RustCore payment instruction into a Kotlin `PaymentInstruction`.
         *
         * @param rustCore The RustCore representation of the payment instruction.
         * @return The Kotlin `PaymentInstruction` object.
         */
        internal fun fromRustCore(rustCore: RustCorePaymentInstruction): PaymentInstruction {
            return PaymentInstruction(rustCore.link, rustCore.instruction)
        }
    }

    /**
     * Converts the Kotlin `PaymentInstruction` into the RustCore equivalent.
     *
     * @return The RustCore representation of the payment instruction.
     */
    internal fun toRustCore(): RustCorePaymentInstruction {
        return RustCorePaymentInstruction(link, instruction)
    }
}

/**
 * Represents the data of an order's instructions in the tbDEX protocol, including payin and payout instructions.
 *
 * @property payin The payin payment instructions for Alice.
 * @property payout The payout payment instructions for Alice.
 */
data class OrderInstructionsData (
    var payin: PaymentInstruction,
    var payout: PaymentInstruction
) {
    companion object {
        /**
         * Converts the RustCore order instructions data into a Kotlin `OrderInstructionsData`.
         *
         * @param rustCore The RustCore representation of order instructions data.
         * @return The Kotlin `OrderInstructionsData`.
         */
        internal fun fromRustCore(rustCore: RustCoreOrderInstructionsData): OrderInstructionsData {
            return OrderInstructionsData(
                PaymentInstruction.fromRustCore(rustCore.payin),
                PaymentInstruction.fromRustCore(rustCore.payout)
            )
        }
    }

    /**
     * Converts the Kotlin `OrderInstructionsData` into the RustCore equivalent.
     *
     * @return The RustCore representation of order instructions data.
     */
    internal fun toRustCore(): RustCoreOrderInstructionsData {
        return RustCoreOrderInstructionsData(payin.toRustCore(), payout.toRustCore())
    }
}

/**
 * Represents an Order Instructions message in the tbDEX protocol.
 *
 * An Order Instructions message is sent by a PFI to Alice, providing
 * detailed instructions on how to make a payin or receive a payout.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property data The data part of the Order Instructions, including payment instructions for payin and payout.
 * @property signature The signature verifying the authenticity and integrity of the Order Instructions message.
 * @property rustCoreOrderInstructions The underlying RustCore representation of the Order Instructions.
 */
data class OrderInstructions private constructor(
    val metadata: MessageMetadata,
    val data: OrderInstructionsData,
    val signature: String,
    internal val rustCoreOrderInstructions: tbdex.sdk.rust.OrderInstructions
): Message, ReplyToMessage {
    companion object {
        /**
         * Creates a new Order Instructions message.
         *
         * @param to The DID of the recipient (Alice).
         * @param from The DID of the sender (the PFI).
         * @param exchangeId The exchange ID shared between Alice and the PFI.
         * @param data The data containing payment instructions for payin and payout.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created Order Instructions message.
         * @throws TbdexException if the creation process fails.
         */
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: OrderInstructionsData,
            protocol: String? = null,
            externalId: String? = null
        ): OrderInstructions {
            try {
                val rustCoreOrderInstructions =
                    tbdex.sdk.rust.OrderInstructions.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
                val rustCoreData = rustCoreOrderInstructions.getData()
                return OrderInstructions(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    OrderInstructionsData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreOrderInstructions
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Parses an Order Instructions message from a JSON string.
         *
         * @param json The JSON string representing the Order Instructions.
         * @return The deserialized Order Instructions message.
         * @throws TbdexException if parsing fails.
         */
        fun fromJsonString(json: String): OrderInstructions {
            try {
                val rustCoreOrderInstructions = tbdex.sdk.rust.OrderInstructions.fromJsonString(json)
                val rustCoreData = rustCoreOrderInstructions.getData()
                return OrderInstructions(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    OrderInstructionsData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreOrderInstructions
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Converts a RustCore Order Instructions into a Kotlin Order Instructions.
         *
         * @param rustCoreOrderInstructions The RustCore representation of the Order Instructions.
         * @return The Kotlin Order Instructions message.
         */
        internal fun fromRustCoreOrderInstructions(rustCoreOrderInstructions: tbdex.sdk.rust.OrderInstructions): OrderInstructions {
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                OrderInstructionsData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreOrderInstructions
            )
        }
    }

    /**
     * Serializes the Order Instructions to a JSON string.
     *
     * @return The serialized JSON string of the Order Instructions.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreOrderInstructions.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Order Instructions using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Order Instructions.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOrderInstructions.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Order Instructions' signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreOrderInstructions.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}