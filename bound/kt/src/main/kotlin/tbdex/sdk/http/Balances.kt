package tbdex.sdk.http

import tbdex.sdk.resources.Balance
import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.GetBalancesResponseBody as RustCoreGetBalancesResponseBody

class GetBalancesResponseBody private constructor(
    val data: List<Balance>,
    private val rustCoreGetBalancesResponseBody: RustCoreGetBalancesResponseBody
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

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
                Balance(it)
            }
            return GetBalancesResponseBody(balances, rustCoreGetBalancesResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreGetBalancesResponseBody.toJsonString()
    }
}