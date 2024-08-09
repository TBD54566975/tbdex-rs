package tbdex.sdk.http

import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.GetOfferingsResponseBody as RustCoreGetOfferingsResponseBody

class GetOfferingsResponseBody private constructor(
    val data: List<Offering>,
    internal val rustCoreGetOfferingsResponseBody: RustCoreGetOfferingsResponseBody
) {
    constructor(offerings: List<Offering>) : this(
        data = offerings,
        rustCoreGetOfferingsResponseBody = RustCoreGetOfferingsResponseBody(
            offerings.map { it.rustCoreOffering }
        )
    )

    companion object {
        fun fromJsonString(json: String): GetOfferingsResponseBody {
            val rustCoreGetOfferingsResponseBody = RustCoreGetOfferingsResponseBody.fromJsonString(json)
            val offerings = rustCoreGetOfferingsResponseBody.getData().data.map {
                Offering.fromRustCoreOffering(it)
            }
            return GetOfferingsResponseBody(offerings, rustCoreGetOfferingsResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetOfferingsResponseBody.toJsonString()
    }
}
