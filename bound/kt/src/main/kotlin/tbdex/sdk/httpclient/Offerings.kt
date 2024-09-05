package tbdex.sdk.httpclient

import tbdex.sdk.TbdexException
import tbdex.sdk.rust.getOfferings as rustCoreGetOfferings
import tbdex.sdk.resources.Offering

fun getOfferings(pfiDidUri: String): List<Offering> {
    try {
        val rustCoreOfferings = rustCoreGetOfferings(pfiDidUri)
        return rustCoreOfferings.map { Offering.fromRustCoreOffering(it) }
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}