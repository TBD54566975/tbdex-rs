package tbdex.sdk.httpclient

import tbdex.sdk.TbdexException
import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.getBalances as rustCoreGetBalances
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

fun getBalances(pfiDidUri: String, bearerDid: BearerDid): List<Balance> {
    try {
        val rustCoreBalances = rustCoreGetBalances(pfiDidUri, RustCoreBearerDid.fromWeb5(bearerDid))
        return rustCoreBalances.map { Balance.fromRustCoreBalance(it) }
    } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
        throw TbdexException.fromRustCore(e)
    }
}