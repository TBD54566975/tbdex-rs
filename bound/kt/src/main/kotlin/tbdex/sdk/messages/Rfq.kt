package tbdex.sdk.messages

import tbdex.sdk.Json
import tbdex.sdk.TbdexException
import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.Rfq as RustCoreRfq
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class Rfq private constructor(
    val metadata: MessageMetadata,
    val data: RfqData,
    val privateData: RfqPrivateData? = null,
    val signature: String,
    internal val rustCoreRfq: RustCoreRfq
): Message {
    companion object {
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

    fun toJsonString(): String {
        try {
            return rustCoreRfq.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verifyOfferingRequirements(offering: Offering) {
        try {
            rustCoreRfq.verifyOfferingRequirements(offering.rustCoreOffering)
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verifyAllPrivateData() {
        try {
            rustCoreRfq.verifyAllPrivateData()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verifyPresentPrivateData() {
        try {
            rustCoreRfq.verifyPresentPrivateData()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreRfq.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreRfq.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
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
