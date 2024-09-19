package tbdex.sdk.messages

import tbdex.sdk.Json
import tbdex.sdk.TbdexException
import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.Rfq as RustCoreRfq
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents a Request for Quote (RFQ) message in the tbDEX protocol.
 *
 * An RFQ is used by Alice to request a quote from a PFI, specifying the payin, payout, and other requirements.
 *
 * @property metadata Metadata about the message, including sender, recipient, and protocol information.
 * @property data The public data part of the RFQ, such as payin, payout, and offering details.
 * @property privateData Optional private data, which may include sensitive information.
 * @property signature The signature verifying the authenticity and integrity of the RFQ message.
 */
data class Rfq private constructor(
    val metadata: MessageMetadata,
    val data: RfqData,
    val privateData: RfqPrivateData? = null,
    val signature: String,
    internal val rustCoreRfq: RustCoreRfq
): Message {
    companion object {
        /**
         * Creates a new RFQ message.
         *
         * @param to The DID of the recipient (the PFI).
         * @param from The DID of the sender (Alice).
         * @param createRfqData The data needed to create the RFQ, such as payin and payout methods.
         * @param protocol Optional protocol version.
         * @param externalId Optional external identifier.
         * @return The newly created RFQ message.
         * @throws TbdexException if the creation process fails.
         */
        fun create(
            to: String,
            from: String,
            createRfqData: CreateRfqData,
            protocol: String? = null,
            externalId: String? = null
        ): Rfq {
            try {
                val jsonSerializedCreateRfqData = Json.stringify(createRfqData)
                val rustCoreRfq = RustCoreRfq.create(to, from, jsonSerializedCreateRfqData, protocol, externalId)
                val rustCoreData = rustCoreRfq.getData()
                val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
                val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
                return Rfq(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    data,
                    privateData,
                    rustCoreData.signature,
                    rustCoreRfq
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Parses an RFQ from a JSON string.
         *
         * @param json The JSON string representing the RFQ.
         * @return The deserialized RFQ message.
         * @throws TbdexException if parsing fails.
         */
        fun fromJsonString(json: String): Rfq {
            try {
                val rustCoreRfq = RustCoreRfq.fromJsonString(json)
                val rustCoreData = rustCoreRfq.getData()
                val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
                val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
                return Rfq(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    data,
                    privateData,
                    rustCoreData.signature,
                    rustCoreRfq
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Converts a RustCore RFQ into a Kotlin RFQ.
         *
         * @param rustCoreRfq The RustCore RFQ instance.
         * @return The Kotlin RFQ message.
         */
        internal fun fromRustCoreRfq(rustCoreRfq: RustCoreRfq): Rfq {
            val rustCoreData = rustCoreRfq.getData()
            val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
            val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
            return Rfq(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                data,
                privateData,
                rustCoreData.signature,
                rustCoreRfq
            )
        }
    }

    /**
     * Serializes the RFQ to a JSON string.
     *
     * @return The serialized JSON string of the RFQ.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreRfq.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies that the RFQ meets the requirements specified by an Offering.
     *
     * @param offering The offering against which the RFQ is being verified.
     * @throws TbdexException if verification fails.
     */
    fun verifyOfferingRequirements(offering: Offering) {
        try {
            rustCoreRfq.verifyOfferingRequirements(offering.rustCoreOffering)
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies all private data within the RFQ.
     *
     * @throws TbdexException if verification fails.
     */
    fun verifyAllPrivateData() {
        try {
            rustCoreRfq.verifyAllPrivateData()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies that all present private data fields in the RFQ match their corresponding hashes.
     *
     * @throws TbdexException if verification fails.
     */
    fun verifyPresentPrivateData() {
        try {
            rustCoreRfq.verifyPresentPrivateData()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the RFQ using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the RFQ.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreRfq.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the RFQ's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreRfq.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}

/**
 * Represents the data required to create an RFQ.
 *
 * @property offeringId The ID of the offering the RFQ refers to.
 * @property payin The selected payin method.
 * @property payout The selected payout method.
 * @property claims A list of claim strings associated with the RFQ.
 */
data class CreateRfqData(
    val offeringId: String,
    val payin: CreateSelectedPayinMethod,
    val payout: CreateSelectedPayoutMethod,
    val claims: List<String>
)

/**
 * Represents the selected payin method for an RFQ.
 *
 * @property kind The kind of the payin method.
 * @property paymentDetails Optional payment details.
 * @property amount The amount being paid in.
 */
data class CreateSelectedPayinMethod(
    val kind: String,
    val paymentDetails: Map<String, Any?>? = null,
    val amount: String
)

/**
 * Represents the selected payout method for an RFQ.
 *
 * @property kind The kind of the payout method.
 * @property paymentDetails Optional payout details.
 */
data class CreateSelectedPayoutMethod(
    val kind: String,
    val paymentDetails: Map<String, Any?>? = null
)

/**
 * Represents the data of an RFQ.
 *
 * @property offeringId The ID of the offering the RFQ refers to.
 * @property payin The selected payin method.
 * @property payout The selected payout method.
 * @property claimsHash The hash of the claims associated with the RFQ.
 */
data class RfqData(
    val offeringId: String,
    val payin: SelectedPayinMethod,
    val payout: SelectedPayoutMethod,
    val claimsHash: String?
)

/**
 * Represents the selected payin method for an RFQ.
 *
 * @property kind The kind of the payin method.
 * @property paymentDetailsHash The hash of the payin details.
 * @property amount The amount being paid in.
 */
data class SelectedPayinMethod(
    val kind: String,
    val paymentDetailsHash: String?,
    val amount: String
)

/**
 * Represents the selected payout method for an RFQ.
 *
 * @property kind The kind of the payout method.
 * @property paymentDetailsHash The hash of the payout details.
 */
data class SelectedPayoutMethod(
    val kind: String,
    val paymentDetailsHash: String?
)

/**
 * Represents the private data in an RFQ, such as sensitive payin and payout details.
 *
 * @property salt The salt used to hash the private data.
 * @property payin Optional private payin details.
 * @property payout Optional private payout details.
 * @property claims Optional claims associated with the RFQ.
 */
data class RfqPrivateData(
    val salt: String,
    val payin: PrivatePaymentDetails?,
    val payout: PrivatePaymentDetails?,
    val claims: List<String>?
)

/**
 * Represents private payment details for payin or payout.
 *
 * @property paymentDetails Optional payment details.
 */
data class PrivatePaymentDetails(
    val paymentDetails: Map<String, Any?>? = null
)
