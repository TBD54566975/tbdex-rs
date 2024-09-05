package tbdex.sdk.http

import tbdex.sdk.TbdexException
import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.GetBalancesResponseBody as RustCoreGetBalancesResponseBody

data class GetBalancesResponseBody private constructor(
    val data: List<Balance>,
    internal val rustCoreGetBalancesResponseBody: RustCoreGetBalancesResponseBody
) {
    constructor(balances: List<Balance>) : this(
        data = balances,
        rustCoreGetBalancesResponseBody = try {
            RustCoreGetBalancesResponseBody(
                balances.map { it.rustCoreBalance }
            )
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    )

    companion object {
        fun fromJsonString(json: String): GetBalancesResponseBody {
            try {
                val rustCoreGetBalancesResponseBody = RustCoreGetBalancesResponseBody.fromJsonString(json)
                val balances = rustCoreGetBalancesResponseBody.getData().data.map {
                    Balance.fromRustCoreBalance(it)
                }
                return GetBalancesResponseBody(balances, rustCoreGetBalancesResponseBody)
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreGetBalancesResponseBody.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}
