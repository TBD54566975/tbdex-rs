package tbdex.sdk.resources

import tbdex.sdk.Json
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.Offering as RustCoreOffering
import web5.sdk.dids.BearerDid
import web5.sdk.vc.pex.PresentationDefinition

class Offering private constructor(
    val metadata: ResourceMetadata,
    val data: OfferingData,
    val signature: String,
    internal val rustCoreOffering: RustCoreOffering
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun create(
            from: String,
            data: OfferingData,
            protocol: String? = null
        ): Offering {
            val jsonSerializedData = Json.stringify(data)
            val rustCoreOffering = RustCoreOffering.create(from, jsonSerializedData, protocol)
            val rustCoreData = rustCoreOffering.getData()
            return Offering(
                rustCoreData.metadata,
                Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, OfferingData::class.java),
                rustCoreData.signature,
                rustCoreOffering
            )
        }

        fun fromJsonString(json: String): Offering {
            val rustCoreOffering = RustCoreOffering.fromJsonString(json)
            val rustCoreData = rustCoreOffering.getData()
            val data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
            return Offering(rustCoreData.metadata, data, rustCoreData.signature, rustCoreOffering)
        }

        internal fun fromRustCoreOffering(rustCoreOffering: RustCoreOffering): Offering {
            val rustCoreData = rustCoreOffering.getData()
            val data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
            return Offering(rustCoreData.metadata, data, rustCoreData.signature, rustCoreOffering)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreOffering.toJsonString()
    }

    fun sign(bearerDid: BearerDid) {
        this.rustCoreOffering.sign(bearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreOffering.verify()
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