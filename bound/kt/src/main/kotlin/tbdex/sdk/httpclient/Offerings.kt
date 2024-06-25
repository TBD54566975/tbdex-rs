package tbdex.sdk.httpclient

import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings

fun getOfferings(pfiDidUri: String): List<Offering> {
    val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
    return rustCoreOfferings.map { Offering(it) }
}