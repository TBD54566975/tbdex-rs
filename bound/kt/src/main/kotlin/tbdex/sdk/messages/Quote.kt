package tbdex.sdk.messages

import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.InnerBearerDid
import tbdex.sdk.rust.PaymentInstructionData as RustCorePaymentInstruction
import tbdex.sdk.rust.QuoteDetailsData as RustCoreQuoteDetails
import tbdex.sdk.rust.Quote as RustCoreQuote
import tbdex.sdk.rust.QuoteDataData as RustCoreQuoteData
import web5.sdk.dids.BearerDid

typealias QuoteData = RustCoreQuoteData
typealias QuoteDetails = RustCoreQuoteDetails
typealias PaymentInstruction = RustCorePaymentInstruction

class Quote private constructor(
    val metadata: MessageMetadata,
    val data: QuoteData,
    val signature: String,
    internal val rustCoreQuote: RustCoreQuote
): Message, ReplyToMessage {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: QuoteData,
            protocol: String? = null,
            externalId: String? = null
        ): Quote {
            val rustCoreQuote = RustCoreQuote.create(to, from, exchangeId, data, protocol, externalId)
            val rustCoreData = rustCoreQuote.getData()
            return Quote(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreQuote)
        }

        fun fromJsonString(json: String): Quote {
            val rustCoreQuote = RustCoreQuote.fromJsonString(json)
            val rustCoreData = rustCoreQuote.getData()
            return Quote(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreQuote)
        }

        internal fun fromRustCoreQuote(rustCoreQuote: RustCoreQuote): Quote {
            val rustCoreData = rustCoreQuote.getData()
            return Quote(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreQuote)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreQuote.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        val innerBearerDid = InnerBearerDid.fromWeb5(bearerDid)
        this.rustCoreQuote.sign(innerBearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreQuote.verify()
    }
}