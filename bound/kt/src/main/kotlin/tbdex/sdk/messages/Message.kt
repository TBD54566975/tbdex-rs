package tbdex.sdk.messages

import tbdex.sdk.rust.MessageKind as RustCoreMessageKind
import tbdex.sdk.rust.MessageMetadataData as RustCoreMessageMetadata

/**
 * Represents the different kinds of messages in the tbDEX protocol.
 */
enum class MessageKind {
    RFQ,                   // Request For Quote
    QUOTE,                 // Quote message
    ORDER,                 // Order message
    ORDER_INSTRUCTIONS,     // Instructions for executing an order
    CANCEL,                // Cancel message
    ORDER_STATUS,          // Order status update
    CLOSE;                 // Close message indicating exchange completion or termination

    companion object {
        /**
         * Converts a RustCore message kind into a Kotlin `MessageKind`.
         *
         * @param rustCore The RustCore representation of the message kind.
         * @return The Kotlin `MessageKind`.
         */
        internal fun fromRustCore(rustCore: RustCoreMessageKind): MessageKind {
            return when (rustCore) {
                RustCoreMessageKind.RFQ -> RFQ
                RustCoreMessageKind.QUOTE -> QUOTE
                RustCoreMessageKind.ORDER -> ORDER
                RustCoreMessageKind.ORDER_INSTRUCTIONS -> ORDER_INSTRUCTIONS
                RustCoreMessageKind.CANCEL -> CANCEL
                RustCoreMessageKind.ORDER_STATUS -> ORDER_STATUS
                RustCoreMessageKind.CLOSE -> CLOSE
            }
        }
    }
}

/**
 * Represents metadata associated with a tbDEX message.
 *
 * @property from The DID of the sender of the message.
 * @property to The DID of the recipient of the message.
 * @property kind The kind of message being sent (e.g., RFQ, QUOTE, etc.).
 * @property id The unique identifier for the message.
 * @property exchangeId The exchange ID shared between Alice and the PFI.
 * @property externalId Optional external ID for additional identification.
 * @property protocol The version of the tbDEX protocol in use.
 * @property createdAt The timestamp when the message was created (in ISO 8601 format).
 */
data class MessageMetadata (
    val from: String,
    val to: String,
    val kind: MessageKind,
    val id: String,
    val exchangeId: String,
    val externalId: String?,
    val protocol: String,
    val createdAt: String
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreMessageMetadata): MessageMetadata {
            return MessageMetadata(
                rustCore.from,
                rustCore.to,
                MessageKind.fromRustCore(rustCore.kind),
                rustCore.id,
                rustCore.exchangeId,
                rustCore.externalId,
                rustCore.protocol,
                rustCore.createdAt
            )
        }
    }
}

/**
 * Marker interface for all tbDEX messages.
 */
interface Message {}