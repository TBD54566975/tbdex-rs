package tbdex.sdk.http

import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.GetBalancesResponseBody as RustCoreGetBalancesResponseBody

data class GetBalancesResponseBody private constructor(
    val data: List<Balance>,
    internal val rustCoreGetBalancesResponseBody: RustCoreGetBalancesResponseBody
) {
    constructor(balances: List<Balance>) : this(
        data = balances,
        rustCoreGetBalancesResponseBody = RustCoreGetBalancesResponseBody(
            balances.map { it.rustCoreBalance }
        )
    )

    companion object {
        fun fromJsonString(json: String): GetBalancesResponseBody {
            val rustCoreGetBalancesResponseBody = RustCoreGetBalancesResponseBody.fromJsonString(json)
            val balances = rustCoreGetBalancesResponseBody.getData().data.map {
                Balance.fromRustCoreBalance(it)
            }
            return GetBalancesResponseBody(balances, rustCoreGetBalancesResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetBalancesResponseBody.toJsonString()
    }
}
