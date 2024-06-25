package tbdex.sdk.messages

import com.fasterxml.jackson.databind.JsonNode
import tbdex.sdk.Json
import tbdex.sdk.resources.Offering
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PresentationDefinition
import tbdex.sdk.rust.Rfq as RustCoreRfq
import tbdex.sdk.rust.CreateRfqDataData as RustCoreCreateRfqData
import tbdex.sdk.rust.CreateSelectedPayinMethodData as RustCoreCreateSelectedPayinMethod
import tbdex.sdk.rust.CreateSelectedPayoutMethodData as RustCoreCreateSelectedPayoutMethod
import tbdex.sdk.rust.RfqDataData as RustCoreRfqData
import tbdex.sdk.rust.SelectedPayinMethodData as RustCoreSelectedPayinMethod
import tbdex.sdk.rust.SelectedPayoutMethodData as RustCoreSelectedPayoutMethod
import tbdex.sdk.rust.RfqPrivateDataData as RustCoreRfqPrivateData
import tbdex.sdk.rust.PrivatePaymentDetailsData as RustCorePrivatePaymentDetails

class Rfq {
    val metadata: MessageMetadata
    val data: RfqData
    val privateData: RfqPrivateData
    val signature: String

    val rustCoreRfq: RustCoreRfq

    constructor(
        bearerDid: BearerDid,
        to: String,
        from: String,
        createRfqData: CreateRfqData,
        protocol: String,
        externalId: String?
    ) {
        this.rustCoreRfq = RustCoreRfq(
            bearerDid.rustCoreBearerDid, to, from, createRfqData.toRustCore(), protocol, externalId
        )

        val rfqData = rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = RfqData.fromRustCore(rfqData.data)
        this.privateData = RfqPrivateData.fromRustCore(rfqData.privateData)
        this.signature = rfqData.signature
    }

    constructor(json: String) {
        this.rustCoreRfq = RustCoreRfq.fromJsonString(json)

        val rfqData = rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = RfqData.fromRustCore(rfqData.data)
        this.privateData = RfqPrivateData.fromRustCore(rfqData.privateData)
        this.signature = rfqData.signature
    }

    constructor(rustCoreRfq: RustCoreRfq) {
        this.rustCoreRfq = rustCoreRfq

        val rfqData = this.rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = RfqData.fromRustCore(rfqData.data)
        this.privateData = RfqPrivateData.fromRustCore(rfqData.privateData)
        this.signature = rfqData.signature
    }

    fun toJson(): String {
        return this.rustCoreRfq.toJson()
    }

    fun verifyOfferingRequirements(offering: Offering): Boolean {
        return this.rustCoreRfq.verifyOfferingRequirements(offering.rustCoreOffering)
    }

    fun verifyAllPrivateData(): Boolean {
        return this.rustCoreRfq.verifyAllPrivateData()
    }

    fun verifyPresentPrivateData(): Boolean {
        return this.rustCoreRfq.verifyPresentPrivateData()
    }
}

data class CreateRfqData(
    val offeringId: String,
    val payin: CreateSelectedPayinMethod,
    val payout: CreateSelectedPayoutMethod,
    val claims: List<String>
) {
    fun toRustCore(): RustCoreCreateRfqData {
        return RustCoreCreateRfqData(
            this.offeringId,
            this.payin.toRustCore(),
            this.payout.toRustCore(),
            this.claims
        )
    }
}

data class CreateSelectedPayinMethod(
    val kind: String,
    val paymentDetails: JsonNode? = null,
    val amount: String
) {
    fun toRustCore(): RustCoreCreateSelectedPayinMethod {
        return RustCoreCreateSelectedPayinMethod(
            this.kind,
            this.paymentDetails?.let { Json.stringify(it) },
            this.amount
        )
    }
}

data class CreateSelectedPayoutMethod(
    val kind: String,
    val paymentDetails: JsonNode? = null
) {
    fun toRustCore(): RustCoreCreateSelectedPayoutMethod {
        return RustCoreCreateSelectedPayoutMethod(
            this.kind,
            this.paymentDetails?.let { Json.stringify(it) }
        )
    }
}

data class RfqData(
    val offeringId: String,
    val payin: SelectedPayinMethod,
    val payout: SelectedPayoutMethod,
    val claimsHash: String?
) {
    companion object {
        internal fun fromRustCore(rustCoreRfqData: RustCoreRfqData): RfqData {
            return RfqData(
                rustCoreRfqData.offeringId,
                SelectedPayinMethod.fromRustCore(rustCoreRfqData.payin),
                SelectedPayoutMethod.fromRustCore(rustCoreRfqData.payout),
                rustCoreRfqData.claimsHash
            )
        }
    }
}

data class SelectedPayinMethod(
    val kind: String,
    val paymentDetailsHash: String?,
    val amount: String
) {
    companion object {
        internal fun fromRustCore(rustCoreSelectedPayinMethod: RustCoreSelectedPayinMethod): SelectedPayinMethod {
            return SelectedPayinMethod(
                rustCoreSelectedPayinMethod.kind,
                rustCoreSelectedPayinMethod.paymentDetailsHash,
                rustCoreSelectedPayinMethod.amount
            )
        }
    }
}

data class SelectedPayoutMethod(
    val kind: String,
    val paymentDetailsHash: String?
) {
    companion object {
        internal fun fromRustCore(rustCoreSelectedPayoutMethod: RustCoreSelectedPayoutMethod): SelectedPayoutMethod {
            return SelectedPayoutMethod(
                rustCoreSelectedPayoutMethod.kind,
                rustCoreSelectedPayoutMethod.paymentDetailsHash
            )
        }
    }
}

data class RfqPrivateData(
    val salt: String,
    val payin: PrivatePaymentDetails?,
    val payout: PrivatePaymentDetails?,
    val claims: List<String>?
) {
    companion object {
        internal fun fromRustCore(rustCoreRfqPrivateData: RustCoreRfqPrivateData): RfqPrivateData {
            return RfqPrivateData(
                rustCoreRfqPrivateData.salt,
                rustCoreRfqPrivateData.payin?.let { PrivatePaymentDetails.fromRustCore(it) },
                rustCoreRfqPrivateData.payout?.let { PrivatePaymentDetails.fromRustCore(it) },
                rustCoreRfqPrivateData.claims
            )
        }
    }
}

data class PrivatePaymentDetails(
    val paymentDetails: JsonNode? = null
) {
    companion object {
        internal fun fromRustCore(rustCorePrivatePaymentDetails: RustCorePrivatePaymentDetails): PrivatePaymentDetails {
            return PrivatePaymentDetails(
                Json.jsonMapper.readTree(rustCorePrivatePaymentDetails.paymentDetails)
            )
        }
    }
}
