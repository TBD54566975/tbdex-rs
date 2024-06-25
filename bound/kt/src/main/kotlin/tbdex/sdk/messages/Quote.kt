package tbdex.sdk.messages

import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.PaymentInstructionsData as RustCorePaymentInstructions
import tbdex.sdk.rust.QuoteDetailsData as RustCoreQuoteDetails
import tbdex.sdk.rust.Quote as RustCoreQuote
import tbdex.sdk.rust.QuoteDataData as RustCoreQuoteData

typealias QuoteData = RustCoreQuoteData
typealias QuoteDetails = RustCoreQuoteDetails
typealias PaymentInstructions = RustCorePaymentInstructions

class Quote {
    val metadata: MessageMetadata
    val data: QuoteData
    val signature: String

    val rustCoreQuote: RustCoreQuote

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        exchangeId: String,
        data: QuoteData,
        protocol: String,
        externalId: String? = null
    ) {
        this.rustCoreQuote = RustCoreQuote(bearerDid.rustCoreBearerDid, to, from, exchangeId, data, protocol, externalId)

        this.metadata = rustCoreQuote.getData().metadata
        this.data = rustCoreQuote.getData().data
        this.signature = rustCoreQuote.getData().signature
    }

    constructor(json: String) {
        this.rustCoreQuote = RustCoreQuote.fromJsonString(json)

        this.metadata = rustCoreQuote.getData().metadata
        this.data = QuoteData(
            this.rustCoreQuote.getData().data.expiresAt,
            this.rustCoreQuote.getData().data.payin,
            this.rustCoreQuote.getData().data.payout,
        )
        this.signature = rustCoreQuote.getData().signature
    }

    constructor(rustCoreQuote: RustCoreQuote) {
        this.rustCoreQuote = rustCoreQuote

        this.metadata = this.rustCoreQuote.getData().metadata
        this.data = this.rustCoreQuote.getData().data
        this.signature = this.rustCoreQuote.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreQuote.toJson()
    }
}