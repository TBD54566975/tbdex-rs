package tbdex.sdk.http

import tbdex.sdk.TbdexException
import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.GetOfferingsResponseBody as RustCoreGetOfferingsResponseBody

data class GetOfferingsResponseBody private constructor(
    val data: List<Offering>,
    internal val rustCoreGetOfferingsResponseBody: RustCoreGetOfferingsResponseBody
) {
    constructor(offerings: List<Offering>) : this(
        offerings,
        try {
            RustCoreGetOfferingsResponseBody(
                offerings.map { it.rustCoreOffering }
            )
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    )

    companion object {
        fun fromJsonString(json: String): GetOfferingsResponseBody {
            try {
                val rustCoreGetOfferingsResponseBody = RustCoreGetOfferingsResponseBody.fromJsonString(json)
                val offerings = rustCoreGetOfferingsResponseBody.getData().data.map {
                    Offering.fromRustCoreOffering(it)
                }
                return GetOfferingsResponseBody(offerings, rustCoreGetOfferingsResponseBody)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreGetOfferingsResponseBody.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}
