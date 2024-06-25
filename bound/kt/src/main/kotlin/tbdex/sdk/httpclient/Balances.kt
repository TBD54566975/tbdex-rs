package tbdex.sdk.httpclient

import tbdex.sdk.resources.Balance
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.getBalances as rustCoreGetBalances

fun getBalances(pfiDidUri: String, bearerDid: BearerDid): List<Balance> {
    val rustCoreBalances = rustCoreGetBalances(pfiDidUri, bearerDid.rustCoreBearerDid)
    return rustCoreBalances.map { Balance(it) }
}