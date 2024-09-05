package tbdex.sdk.resources

import tbdex.sdk.Json
import tbdex.sdk.TbdexException
import tbdex.sdk.rust.Offering as RustCoreOffering
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid
import web5.sdk.vc.pex.PresentationDefinition

data class Offering private constructor(
    val metadata: ResourceMetadata,
    val data: OfferingData,
    val signature: String,
    internal val rustCoreOffering: RustCoreOffering
) {
    companion object {
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

    fun toJsonString(): String {
        try {
            return rustCoreOffering.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOffering.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreOffering.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}

data class OfferingData(
    val description: String,
    val payoutUnitsPerPayinUnit: String,
    val payin: PayinDetails,
    val payout: PayoutDetails,
    val requiredClaims: PresentationDefinition?,
    val cancellation: CancellationDetails
)

data class PayinDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayinMethod>
)

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

data class PayoutDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayoutMethod>
)

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

data class CancellationDetails(
    val enabled: Boolean,
    val termsUrl: String? = null,
    val terms: String? = null
)