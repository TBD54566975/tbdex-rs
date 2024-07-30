package tbdex.sdk.messages

import tbdex.sdk.Json
import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.InternalBearerDid
import tbdex.sdk.rust.Rfq as RustCoreRfq
import web5.sdk.dids.BearerDid

class Rfq private constructor(
    val metadata: MessageMetadata,
    val data: RfqData,
    val privateData: RfqPrivateData? = null,
    val signature: String,
    internal val rustCoreRfq: RustCoreRfq
): Message {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun create(
            to: String,
            from: String,
            createRfqData: CreateRfqData,
            protocol: String? = null,
            externalId: String? = null
        ): Rfq {
            val jsonSerializedCreateRfqData = Json.stringify(createRfqData)
            val rustCoreRfq = RustCoreRfq.create(to, from, jsonSerializedCreateRfqData, protocol, externalId)
            val rustCoreData = rustCoreRfq.getData()
            val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
            val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
            return Rfq(rustCoreData.metadata, data, privateData, rustCoreData.signature, rustCoreRfq)
        }

        fun fromJsonString(json: String): Rfq {
            val rustCoreRfq = RustCoreRfq.fromJsonString(json)
            val rustCoreData = rustCoreRfq.getData()
            val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
            val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
            return Rfq(rustCoreData.metadata, data, privateData, rustCoreData.signature, rustCoreRfq)
        }

        internal fun fromRustCoreRfq(rustCoreRfq: RustCoreRfq): Rfq {
            val rustCoreData = rustCoreRfq.getData()
            val data = Json.jsonMapper.readValue(rustCoreData.jsonSerializedData, RfqData::class.java)
            val privateData = rustCoreData.jsonSerializedPrivateData?.let { Json.jsonMapper.readValue(it, RfqPrivateData::class.java) }
            return Rfq(rustCoreData.metadata, data, privateData, rustCoreData.signature, rustCoreRfq)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreRfq.toJsonString()
    }

    fun verifyOfferingRequirements(offering: Offering) {
        this.rustCoreRfq.verifyOfferingRequirements(offering.rustCoreOffering)
    }

    fun verifyAllPrivateData() {
        this.rustCoreRfq.verifyAllPrivateData()
    }

    fun verifyPresentPrivateData() {
        this.rustCoreRfq.verifyPresentPrivateData()
    }

    fun sign(bearerDid: BearerDid) {
        val internalBearerDid = InternalBearerDid.fromWeb5(bearerDid)
        this.rustCoreRfq.sign(internalBearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreRfq.verify()
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
