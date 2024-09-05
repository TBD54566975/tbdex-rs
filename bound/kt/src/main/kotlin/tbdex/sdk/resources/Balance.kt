package tbdex.sdk.resources

import tbdex.sdk.TbdexException
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

data class BalanceData (
    val currencyCode: String,
    val available: String
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreBalanceData): BalanceData {
            return BalanceData(rustCore.currencyCode, rustCore.available)
        }
    }

    internal fun toRustCore(): RustCoreBalanceData {
        return RustCoreBalanceData(currencyCode, available)
    }
}

data class Balance private constructor(
    val metadata: ResourceMetadata,
    val data: BalanceData,
    val signature: String,
    internal val rustCoreBalance: RustCoreBalance
){
    companion object {
        fun create(
            from: String,
            data: BalanceData,
            protocol: String? = null
        ): Balance {
            try {
                val rustCoreBalance = RustCoreBalance.create(from, data.toRustCore(), protocol)
                val rustCoreData = rustCoreBalance.getData()
                return Balance(
                    ResourceMetadata.fromRustCore(rustCoreData.metadata),
                    BalanceData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreBalance
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        fun fromJsonString(json: String): Balance {
            try {
                val rustCoreBalance = RustCoreBalance.fromJsonString(json)
                val rustCoreData = rustCoreBalance.getData()
                return Balance(
                    ResourceMetadata.fromRustCore(rustCoreData.metadata),
                    BalanceData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreBalance
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreBalance(rustCoreBalance: RustCoreBalance): Balance {
            val rustCoreData = rustCoreBalance.getData()
            return Balance(
                ResourceMetadata.fromRustCore(rustCoreData.metadata),
                BalanceData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreBalance
            )
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreBalance.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreBalance.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreBalance.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}