package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.PaymentInstructionData as RustCorePaymentInstruction
import tbdex.sdk.rust.QuoteDetailsData as RustCoreQuoteDetails
import tbdex.sdk.rust.Quote as RustCoreQuote
import tbdex.sdk.rust.QuoteDataData as RustCoreQuoteData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class QuoteDetails (
    val currencyCode: String,
    val subtotal: String,
    val total: String,
    val fee: String?
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreQuoteDetails): QuoteDetails {
            return QuoteDetails(
                rustCore.currencyCode,
                rustCore.subtotal,
                rustCore.total,
                rustCore.fee
            )
        }
    }

    internal fun toRustCore(): RustCoreQuoteDetails {
        return RustCoreQuoteDetails(currencyCode, subtotal, total, fee)
    }
}

data class QuoteData (
    val expiresAt: String,
    val payoutUnitsPerPayinUnit: String,
    val payin: QuoteDetails,
    val payout: QuoteDetails
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreQuoteData): QuoteData {
            return QuoteData(
                rustCore.expiresAt,
                rustCore.payoutUnitsPerPayinUnit,
                QuoteDetails.fromRustCore(rustCore.payin),
                QuoteDetails.fromRustCore(rustCore.payout)
            )
        }
    }

    internal fun toRustCore(): RustCoreQuoteData {
        return RustCoreQuoteData(
            expiresAt,
            payoutUnitsPerPayinUnit,
            payin.toRustCore(),
            payout.toRustCore()
        )
    }
}

class Quote private constructor(
    val metadata: MessageMetadata,
    val data: QuoteData,
    val signature: String,
    internal val rustCoreQuote: RustCoreQuote
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: QuoteData,
            protocol: String? = null,
            externalId: String? = null
        ): Quote {
            val rustCoreQuote = RustCoreQuote.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
            val rustCoreData = rustCoreQuote.getData()
            return Quote(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                QuoteData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreQuote
            )
        }

        fun fromJsonString(json: String): Quote {
            val rustCoreQuote = RustCoreQuote.fromJsonString(json)
            val rustCoreData = rustCoreQuote.getData()
            return Quote(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                QuoteData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreQuote
            )
        }

        internal fun fromRustCoreQuote(rustCoreQuote: RustCoreQuote): Quote {
            val rustCoreData = rustCoreQuote.getData()
            return Quote(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                QuoteData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreQuote
            )
        }
    }

    fun toJsonString(): String {
        return this.rustCoreQuote.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreQuote.sign(RustCoreBearerDid.fromWeb5(bearerDid))
    }

    fun verify() {
        this.rustCoreQuote.verify()
    }
}