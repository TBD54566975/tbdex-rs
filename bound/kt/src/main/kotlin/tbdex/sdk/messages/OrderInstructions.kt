package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.PaymentInstructionData as RustCorePaymentInstruction
import tbdex.sdk.rust.OrderInstructionsDataData as RustCoreOrderInstructionsData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class PaymentInstruction (
    val link: String?,
    val instruction: String?
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCorePaymentInstruction): PaymentInstruction {
            return PaymentInstruction(rustCore.link, rustCore.instruction)
        }
    }

    internal fun toRustCore(): RustCorePaymentInstruction {
        return RustCorePaymentInstruction(link, instruction)
    }
}

data class OrderInstructionsData (
    var payin: PaymentInstruction,
    var payout: PaymentInstruction
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreOrderInstructionsData): OrderInstructionsData {
            return OrderInstructionsData(
                PaymentInstruction.fromRustCore(rustCore.payin),
                PaymentInstruction.fromRustCore(rustCore.payout)
            )
        }
    }

    internal fun toRustCore(): RustCoreOrderInstructionsData {
        return RustCoreOrderInstructionsData(payin.toRustCore(), payout.toRustCore())
    }
}

data class OrderInstructions private constructor(
    val metadata: MessageMetadata,
    val data: OrderInstructionsData,
    val signature: String,
    internal val rustCoreOrderInstructions: tbdex.sdk.rust.OrderInstructions
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: OrderInstructionsData,
            protocol: String? = null,
            externalId: String? = null
        ): OrderInstructions {
            val rustCoreOrderInstructions =
                tbdex.sdk.rust.OrderInstructions.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                OrderInstructionsData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreOrderInstructions
            )
        }

        fun fromJsonString(json: String): OrderInstructions {
            val rustCoreOrderInstructions = tbdex.sdk.rust.OrderInstructions.fromJsonString(json)
            val rustCoreData = rustCoreOrderInstructions.getData()
            return OrderInstructions(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                OrderInstructionsData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreOrderInstructions
            )
        }

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

    fun toJsonString(): String {
        return this.rustCoreOrderInstructions.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreOrderInstructions.sign(RustCoreBearerDid.fromWeb5(bearerDid))
    }

    fun verify() {
        this.rustCoreOrderInstructions.verify()
    }
}