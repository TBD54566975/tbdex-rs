package tbdex.sdk.messages

import tbdex.sdk.rust.MessageKind as RustCoreMessageKind
import tbdex.sdk.rust.MessageMetadataData as RustCoreMessageMetadata

enum class MessageKind {
    RFQ,
    QUOTE,
    ORDER,
    ORDER_INSTRUCTIONS,
    CANCEL,
    ORDER_STATUS,
    CLOSE;

    companion object {
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

interface Message {}