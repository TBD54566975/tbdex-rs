package tbdex.sdk.httpclient

import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings
import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.SystemArchitecture

fun getOfferings(pfiDidUri: String): List<Offering> {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
    return rustCoreOfferings.map { Offering.fromRustCoreOffering(it) }
}