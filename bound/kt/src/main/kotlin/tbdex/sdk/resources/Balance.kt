package tbdex.sdk.resources

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData

typealias BalanceData = RustCoreBalanceData

class Balance private constructor(
    val metadata: ResourceMetadata,
    val data: BalanceData,
    val signature: String,
    internal val rustCoreBalance: RustCoreBalance
){
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun create(
            bearerDid: BearerDid,
            from: String,
            data: BalanceData,
            signature: String
        ): Balance {
            val rustCoreBalance = RustCoreBalance(bearerDid.rustCoreBearerDid, from, data, signature)
            val rustCoreData = rustCoreBalance.getData()
            return Balance(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreBalance)
        }

        fun fromJsonString(json: String): Balance {
            val rustCoreBalance = RustCoreBalance.fromJsonString(json)
            val rustCoreData = rustCoreBalance.getData()
            return Balance(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreBalance)
        }

        internal fun fromRustCoreBalance(rustCoreBalance: RustCoreBalance): Balance {
            val rustCoreData = rustCoreBalance.getData()
            return Balance(rustCoreData.metadata, rustCoreData.data, rustCoreData.signature, rustCoreBalance)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreBalance.toJsonString()
    }
}