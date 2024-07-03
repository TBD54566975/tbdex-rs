package tbdex.sdk.messages

import tbdex.sdk.Json
import tbdex.sdk.resources.Offering
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Rfq as RustCoreRfq

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
        val jsonSerializedCreateRfqData = Json.stringify(createRfqData)
        this.rustCoreRfq = RustCoreRfq(
            bearerDid.rustCoreBearerDid, to, from, jsonSerializedCreateRfqData, protocol, externalId
        )

        val rfqData = rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = Json.jsonMapper.readValue(rfqData.jsonSerializedData, RfqData::class.java)
        this.privateData = Json.jsonMapper.readValue(rfqData.jsonSerializedPrivateData, RfqPrivateData::class.java)
        this.signature = rfqData.signature
    }

    constructor(json: String, requireAllPrivateData: Boolean = false) {
        this.rustCoreRfq = RustCoreRfq.fromJsonString(json, requireAllPrivateData)

        val rfqData = rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = Json.jsonMapper.readValue(rfqData.jsonSerializedData, RfqData::class.java)
        this.privateData = Json.jsonMapper.readValue(rfqData.jsonSerializedPrivateData, RfqPrivateData::class.java)
        this.signature = rfqData.signature
    }

    constructor(rustCoreRfq: RustCoreRfq) {
        this.rustCoreRfq = rustCoreRfq

        val rfqData = this.rustCoreRfq.getData()
        this.metadata = rfqData.metadata
        this.data = Json.jsonMapper.readValue(rfqData.jsonSerializedData, RfqData::class.java)
        this.privateData = Json.jsonMapper.readValue(rfqData.jsonSerializedPrivateData, RfqPrivateData::class.java)
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
)

data class CreateSelectedPayinMethod(
    val kind: String,
    val paymentDetails: Map<String, Any?>? = null,
    val amount: String
)

data class CreateSelectedPayoutMethod(
    val kind: String,
    val paymentDetails: Map<String, Any?>? = null
)

data class RfqData(
    val offeringId: String,
    val payin: SelectedPayinMethod,
    val payout: SelectedPayoutMethod,
    val claimsHash: String?
)

data class SelectedPayinMethod(
    val kind: String,
    val paymentDetailsHash: String?,
    val amount: String
)

data class SelectedPayoutMethod(
    val kind: String,
    val paymentDetailsHash: String?
)

data class RfqPrivateData(
    val salt: String,
    val payin: PrivatePaymentDetails?,
    val payout: PrivatePaymentDetails?,
    val claims: List<String>?
)

data class PrivatePaymentDetails(
    val paymentDetails: Map<String, Any?>? = null
)
