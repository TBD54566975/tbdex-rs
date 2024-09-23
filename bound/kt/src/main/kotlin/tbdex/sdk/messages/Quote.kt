package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.PaymentInstructionData as RustCorePaymentInstruction
import tbdex.sdk.rust.QuoteDetailsData as RustCoreQuoteDetails
import tbdex.sdk.rust.Quote as RustCoreQuote
import tbdex.sdk.rust.QuoteDataData as RustCoreQuoteData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents the details of a quote in the tbDEX protocol, including the currency, subtotal, total, and optional fee.
 *
 * @property currencyCode The currency code (ISO 4217 format) of the quote.
 * @property subtotal The subtotal amount of the transaction, excluding fees.
 * @property total The total amount of the transaction, including fees.
 * @property fee Optional fee associated with the transaction.
 */
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

/**
 * Represents the data of a quote in the tbDEX protocol.
 *
 * @property expiresAt The expiration date of the quote.
 * @property payoutUnitsPerPayinUnit The exchange rate for the quote, indicating how many payout units are provided per payin unit.
 * @property payin The details of the payin currency and amount.
 * @property payout The details of the payout currency and amount.
 */
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

/**
 * Represents a Quote message in the tbDEX protocol.
 *
 * A Quote message is sent by a PFI to Alice in response to an RFQ (Request for Quote),
 * detailing the exchange rate, fees, and other details for a potential exchange.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property data The data part of the Quote, containing the exchange rate, payin, and payout details.
 * @property signature The signature verifying the authenticity and integrity of the Quote message.
 * @property rustCoreQuote The underlying RustCore representation of the Quote.
 */
data class Quote private constructor(
    val metadata: MessageMetadata,
    val data: QuoteData,
    val signature: String,
    internal val rustCoreQuote: RustCoreQuote
): Message, ReplyToMessage {
    companion object {
        /**
         * Creates a new Quote message.
         *
         * @param to The DID of the recipient (Alice).
         * @param from The DID of the sender (the PFI).
         * @param exchangeId The ID of the exchange between Alice and the PFI.
         * @param data The data containing the quote details, such as exchange rate and fees.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created Quote message.
         * @throws TbdexException if the creation process fails.
         */
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: QuoteData,
            protocol: String? = null,
            externalId: String? = null
        ): Quote {
            try {
                val rustCoreQuote = RustCoreQuote.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
                val rustCoreData = rustCoreQuote.getData()
                return Quote(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    QuoteData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreQuote
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Parses a Quote from a JSON string.
         *
         * @param json The JSON string representing the Quote.
         * @return The deserialized Quote message.
         * @throws TbdexException if parsing fails.
         */
        fun fromJsonString(json: String): Quote {
            try {
                val rustCoreQuote = RustCoreQuote.fromJsonString(json)
                val rustCoreData = rustCoreQuote.getData()
                return Quote(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    QuoteData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreQuote
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
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

    /**
     * Serializes the Quote to a JSON string.
     *
     * @return The serialized JSON string of the Quote.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreQuote.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Quote using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Quote.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreQuote.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Quote's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreQuote.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}
