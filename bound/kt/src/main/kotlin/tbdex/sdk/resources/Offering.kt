package tbdex.sdk.resources

import com.fasterxml.jackson.databind.JsonNode
import tbdex.sdk.Json
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PresentationDefinition
import tbdex.sdk.rust.Offering as RustCoreOffering
import tbdex.sdk.rust.OfferingDataData as RustCoreOfferingData
import tbdex.sdk.rust.PayinDetailsData as RustCorePayinDetails
import tbdex.sdk.rust.PayinMethodData as RustCorePayinMethod
import tbdex.sdk.rust.PayoutDetailsData as RustCorePayoutDetails
import tbdex.sdk.rust.PayoutMethodData as RustCorePayoutMethod

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
        this.rustCoreOffering = RustCoreOffering(bearerDid.rustCoreBearerDid, from, data.toRustCore(), protocol)

        this.metadata = rustCoreOffering.getData().metadata
        this.data = OfferingData.fromRustCore(rustCoreOffering.getData().data)
        this.signature = rustCoreOffering.getData().signature
    }

    constructor(json: String) {
        this.rustCoreOffering = RustCoreOffering.fromJsonString(json)

        this.metadata = rustCoreOffering.getData().metadata
        this.data = OfferingData.fromRustCore(rustCoreOffering.getData().data)
        this.signature = rustCoreOffering.getData().signature
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
) {
    fun toRustCore(): RustCoreOfferingData {
        return RustCoreOfferingData(
            this.description,
            this.payoutUnitsPerPayinUnit,
            this.payin.toRustCore(),
            this.payout.toRustCore(),
            this.requiredClaims?.rustCorePresentationDefinition?.getData()
        )
    }

    companion object {
        internal fun fromRustCore (rustCoreOfferingData: RustCoreOfferingData): OfferingData {
            return OfferingData(
                rustCoreOfferingData.description,
                rustCoreOfferingData.payoutUnitsPerPayinUnit,
                PayinDetails.fromRustCore(rustCoreOfferingData.payin),
                PayoutDetails.fromRustCore(rustCoreOfferingData.payout),
                rustCoreOfferingData.requiredClaims?.let { PresentationDefinition(it) }
            )
        }
    }
}

data class PayinDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayinMethod>
) {
    fun toRustCore(): RustCorePayinDetails {
        return RustCorePayinDetails(
            this.currencyCode,
            this.min,
            this.max,
            this.methods.map { it.toRustCore() }
        )
    }

    companion object {
        internal fun fromRustCore(rustCorePayinDetails: RustCorePayinDetails): PayinDetails {
            return PayinDetails(
                rustCorePayinDetails.currencyCode,
                rustCorePayinDetails.min,
                rustCorePayinDetails.max,
                rustCorePayinDetails.methods.map { PayinMethod.fromRustCore(it) }
            )
        }
    }
}

data class PayinMethod(
    val kind: String,
    val name: String? = null,
    val description: String? = null,
    val group: String? = null,
    val requiredPaymentDetails: JsonNode? = null,
    val fee: String? = null,
    val min: String? = null,
    val max: String? = null
) {
    fun toRustCore(): RustCorePayinMethod {
        return RustCorePayinMethod(
            this.kind,
            this.name,
            this.description,
            this.group,
            this.requiredPaymentDetails?.let { Json.stringify(it) },
            this.fee,
            this.min,
            this.max
        )
    }

    companion object {
        internal fun fromRustCore(rustCorePayinMethod: RustCorePayinMethod): PayinMethod {
            return PayinMethod(
                rustCorePayinMethod.kind,
                rustCorePayinMethod.name,
                rustCorePayinMethod.description,
                rustCorePayinMethod.group,
                rustCorePayinMethod.requiredPaymentDetails?.let { Json.jsonMapper.readTree(it) },
                rustCorePayinMethod.fee,
                rustCorePayinMethod.min,
                rustCorePayinMethod.max,
            )
        }
    }
}

data class PayoutDetails(
    val currencyCode: String,
    val min: String? = null,
    val max: String? = null,
    val methods: List<PayoutMethod>
) {
    fun toRustCore(): RustCorePayoutDetails {
        return RustCorePayoutDetails(
            this.currencyCode,
            this.min,
            this.max,
            this.methods.map { it.toRustCore() }
        )
    }

    companion object {
        internal fun fromRustCore(rustCorePayoutDetails: RustCorePayoutDetails): PayoutDetails {
            return PayoutDetails(
                rustCorePayoutDetails.currencyCode,
                rustCorePayoutDetails.min,
                rustCorePayoutDetails.max,
                rustCorePayoutDetails.methods.map { PayoutMethod.fromRustCore(it) }
            )
        }
    }
}

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
) {
    fun toRustCore(): RustCorePayoutMethod {
        return RustCorePayoutMethod(
            this.kind,
            this.name,
            this.description,
            this.group,
            this.requiredPaymentDetails?.let { Json.stringify(it) },
            this.fee,
            this.min,
            this.max,
            this.estimatedSettlementTime
        )
    }

    companion object {
        internal fun fromRustCore(rustCorePayoutMethod: RustCorePayoutMethod): PayoutMethod {
            return PayoutMethod(
                rustCorePayoutMethod.kind,
                rustCorePayoutMethod.name,
                rustCorePayoutMethod.description,
                rustCorePayoutMethod.group,
                rustCorePayoutMethod.requiredPaymentDetails?.let { Json.jsonMapper.readTree(it) },
                rustCorePayoutMethod.fee,
                rustCorePayoutMethod.min,
                rustCorePayoutMethod.max,
                rustCorePayoutMethod.estimatedSettlementTime
            )
        }
    }
}