package tbdex.sdk.httpclient

import tbdex.sdk.resources.Offering
import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings
import tbdex.sdk.rust.MultiArchitecture as RustCore

fun getOfferings(pfiDidUri: String): List<Offering> {
//    val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
    val rustCoreOfferings = RustCore.getOfferings(pfiDidUri)
    return rustCoreOfferings.map { Offering(it) }
}