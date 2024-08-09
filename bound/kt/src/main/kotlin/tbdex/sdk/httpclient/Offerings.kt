package tbdex.sdk.httpclient

import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings
import tbdex.sdk.resources.Offering

fun getOfferings(pfiDidUri: String): List<Offering> {
    val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
    return rustCoreOfferings.map { Offering.fromRustCoreOffering(it) }
}