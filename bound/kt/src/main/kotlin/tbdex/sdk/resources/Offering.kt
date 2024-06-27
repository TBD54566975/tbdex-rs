package tbdex.sdk.resources

import com.fasterxml.jackson.databind.JsonNode
import tbdex.sdk.Json
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PresentationDefinition
import tbdex.sdk.rust.Offering as RustCoreOffering

class Offering {
    val metadata: ResourceMetadata
    val data: OfferingData
    val signature: String

    val rustCoreOffering: RustCoreOffering

    constructor(
        bearerDid: BearerDid,
        from: String,
        data: OfferingData,
        protocol: String
    ) {
        val jsonSerializedData = Json.stringify(data)
        this.rustCoreOffering = RustCoreOffering(bearerDid.rustCoreBearerDid, from, jsonSerializedData, protocol)

        this.metadata = rustCoreOffering.getData().metadata
        this.data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
        this.signature = rustCoreOffering.getData().signature
    }

    constructor(json: String) {
        this.rustCoreOffering = RustCoreOffering.fromJsonString(json)

        this.metadata = rustCoreOffering.getData().metadata
        this.data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
        this.signature = rustCoreOffering.getData().signature
    }

    constructor(rustCoreOffering: RustCoreOffering) {
        this.rustCoreOffering = rustCoreOffering

        this.metadata = this.rustCoreOffering.getData().metadata
        this.data = Json.jsonMapper.readValue(rustCoreOffering.getData().jsonSerializedData, OfferingData::class.java)
        this.signature = this.rustCoreOffering.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreOffering.toJson()
    }
}

data class OfferingData(
    val description: String,
    val payoutUnitsPerPayinUnit: String,
    val payin: PayinDetails,
    val payout: PayoutDetails,
    val requiredClaims: PresentationDefinition?
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
    val requiredPaymentDetails: JsonNode? = null,
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
    val requiredPaymentDetails: JsonNode? = null,
    val fee: String? = null,
    val min: String? = null,
    val max: String? = null,
    val estimatedSettlementTime: Long
)