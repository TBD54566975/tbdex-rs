package tbdex.sdk.httpclient

import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.getBalances as rustCoreGetBalances
import web5.sdk.dids.BearerDid

fun getBalances(pfiDidUri: String, bearerDid: BearerDid): List<Balance> {
    val rustCoreBalances = rustCoreGetBalances(pfiDidUri, bearerDid.rustCoreBearerDid)
    return rustCoreBalances.map { Balance.fromRustCoreBalance(it) }
}