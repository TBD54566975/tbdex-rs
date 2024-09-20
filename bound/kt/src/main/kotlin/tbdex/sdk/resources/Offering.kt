package tbdex.sdk.resources

import tbdex.sdk.Json
import tbdex.sdk.TbdexException
import tbdex.sdk.rust.Offering as RustCoreOffering
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid
import web5.sdk.vc.pex.PresentationDefinition

/**
 * Represents an Offering resource in the tbDEX protocol.
 *
 * An Offering is created by a PFI to define the exchange parameters, such as payin/payout details,
 * exchange rates, required credentials, and cancellation policies.
 *
 * @property metadata Metadata about the resource, including sender and resource type.
 * @property data The details of the offering, including payin/payout methods, required claims, and cancellation.
 * @property signature The signature verifying the authenticity and integrity of the Offering resource.
 * @property rustCoreOffering The underlying RustCore representation of the Offering resource.
 */
data class Offering private constructor(
    val metadata: ResourceMetadata,
    val data: OfferingData,
    val signature: String,
    internal val rustCoreOffering: RustCoreOffering
) {
    companion object {
        /**
         * Creates a new Offering resource.
         *
         * @param from The DID of the sender (the PFI).
         * @param data The offering details including exchange rates, payin/payout details, and required claims.
         * @param protocol Optional protocol version.
         * @return The newly created Offering resource.
         * @throws TbdexException if the creation process fails.
         */
        fun create(
            from: String,
            data: OfferingData,
            protocol: String? = null
        ): Offering {
            try {
                val jsonSerializedData = Json.stringify(data)
                val rustCoreOffering = RustCoreOffering.create(from, jsonSerializedData, protocol)
                val rustCoreData = rustCoreOffering.getData()
                return Offering(
                    ResourceMetadata.fromRustCore(rustCoreData.metadata),
                    Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, OfferingData::class.java),
                    rustCoreData.signature,
                    rustCoreOffering
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        /**
         * Parses an Offering resource from a JSON string.
         *
         * @param json The JSON string representing the Offering resource.
         * @return The deserialized Offering resource.
         * @throws TbdexException if parsing fails.
         */
        fun fromJsonString(json: String): Offering {
            try {
                val rustCoreOffering = RustCoreOffering.fromJsonString(json)
                val rustCoreData = rustCoreOffering.getData()
                val data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
                return Offering(
                    ResourceMetadata.fromRustCore(rustCoreData.metadata),
                    data,
                    rustCoreData.signature,
                    rustCoreOffering
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreOffering(rustCoreOffering: RustCoreOffering): Offering {
            val rustCoreData = rustCoreOffering.getData()
            val data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
            return Offering(
                ResourceMetadata.fromRustCore(rustCoreData.metadata),
                data,
                rustCoreData.signature,
                rustCoreOffering
            )
        }
    }

    /**
     * Serializes the Offering resource to a JSON string.
     *
     * @return The serialized JSON string of the Offering resource.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreOffering.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Offering resource using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Offering resource.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOffering.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Offering resource's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreOffering.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}

/**
 * Represents the detailed data for an Offering resource in the tbDEX protocol.
 *
 * @property description A brief description of what is being offered.
 * @property payoutUnitsPerPayinUnit The exchange rate, indicating how many payout units are provided per payin unit.
 * @property payin The details of the payin currency and available methods.
 * @property payout The details of the payout currency and available methods.
 * @property requiredClaims Optional claims required to participate in the offering.
 * @property cancellation The details of the offering's cancellation policy.
 */
data class OfferingData(
    val description: String,
    val payoutUnitsPerPayinUnit: String,
    val payin: PayinDetails,
    val payout: PayoutDetails,
    val requiredClaims: PresentationDefinition?,
    val cancellation: CancellationDetails
)

/**
 * Represents the details of the payin for an Offering.
 *
 * @property currencyCode The ISO 4217 currency code for the payin.
 * @property min Optional minimum payin amount.
 * @property max Optional maximum payin amount.
 * @property methods A list of available methods for making the payin.
 */
data class PayinDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayinMethod>
)

/**
 * Represents a method for making a payin in an Offering.
 *
 * @property kind The kind of payment method.
 * @property name Optional name of the payment method.
 * @property description Optional description of the payment method.
 * @property group Optional group categorization for the payment method.
 * @property requiredPaymentDetails Optional JSON schema specifying required payment details.
 * @property fee Optional fee associated with the payment method.
 * @property min Optional minimum amount for using the payment method.
 * @property max Optional maximum amount for using the payment method.
 */
data class PayinMethod(
    val kind: String,
    val name: String? = null,
    val description: String? = null,
    val group: String? = null,
    val requiredPaymentDetails: Map<String, Any?>? = null,
    val fee: String? = null,
    val min: String? = null,
    val max: String? = null
)

/**
 * Represents the details of the payout for an Offering.
 *
 * @property currencyCode The ISO 4217 currency code for the payout.
 * @property min Optional minimum payout amount.
 * @property max Optional maximum payout amount.
 * @property methods A list of available methods for receiving the payout.
 */
data class PayoutDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayoutMethod>
)

/**
 * Represents a method for receiving a payout in an Offering.
 *
 * @property kind The kind of payout method.
 * @property name Optional name of the payout method.
 * @property description Optional description of the payout method.
 * @property group Optional group categorization for the payout method.
 * @property requiredPaymentDetails Optional JSON schema specifying required payment details.
 * @property fee Optional fee associated with the payout method.
 * @property min Optional minimum amount for using the payout method.
 * @property max Optional maximum amount for using the payout method.
 * @property estimatedSettlementTime The estimated time (in seconds) for the payout to be settled.
 */
data class PayoutMethod(
    val kind: String,
    val name: String? = null,
    val description: String? = null,
    val group: String? = null,
    val requiredPaymentDetails: Map<String, Any?>? = null,
    val fee: String? = null,
    val min: String? = null,
    val max: String? = null,
    val estimatedSettlementTime: Long
)

/**
 * Represents the cancellation policy for an Offering.
 *
 * @property enabled Whether cancellation is enabled for the offering.
 * @property termsUrl Optional URL to a page that describes the terms of cancellation.
 * @property terms Optional human-readable description of the cancellation terms.
 */
data class CancellationDetails(
    val enabled: Boolean,
    val termsUrl: String? = null,
    val terms: String? = null
)