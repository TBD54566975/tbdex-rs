package tbdex.sdk.httpclient

import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.InnerBearerDid
import tbdex.sdk.rust.getBalances as rustCoreGetBalances
import web5.sdk.dids.BearerDid

fun getBalances(pfiDidUri: String, bearerDid: BearerDid): List<Balance> {
    SystemArchitecture.set() // ensure the sys arch is set for first-time loading

    val innerBearerDid = InnerBearerDid.fromWeb5(bearerDid)
    val rustCoreBalances = rustCoreGetBalances(pfiDidUri, innerBearerDid.rustCoreBearerDid)
    return rustCoreBalances.map { Balance.fromRustCoreBalance(it) }
}