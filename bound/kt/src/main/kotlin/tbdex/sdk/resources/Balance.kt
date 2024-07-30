package tbdex.sdk.resources

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.InternalBearerDid
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData
import web5.sdk.dids.BearerDid

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
            from: String,
            data: BalanceData,
            protocol: String? = null
        ): Balance {
            val rustCoreBalance = RustCoreBalance.create(from, data, protocol)
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

    fun sign(bearerDid: BearerDid) {
        val internalBearerDid = InternalBearerDid.fromWeb5(bearerDid)
        this.rustCoreBalance.sign(internalBearerDid.rustCoreBearerDid)
    }

    fun verify() {
        this.rustCoreBalance.verify()
    }
}