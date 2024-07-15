package tbdex.sdk.resources

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData

typealias BalanceData = RustCoreBalanceData

class Balance {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val metadata: ResourceMetadata
    val data: BalanceData
    val signature: String

    val rustCoreBalance: RustCoreBalance

    constructor(
        bearerDid: BearerDid,
        from: String,
        data: BalanceData,
        signature: String
    ) {
        this.rustCoreBalance = RustCoreBalance(bearerDid.rustCoreBearerDid, from, data, signature)

        this.metadata = rustCoreBalance.getData().metadata
        this.data = rustCoreBalance.getData().data
        this.signature = rustCoreBalance.getData().signature
    }

    constructor(json: String) {
        this.rustCoreBalance = RustCoreBalance.fromJsonString(json)

        this.metadata = rustCoreBalance.getData().metadata
        this.data = BalanceData(
            this.rustCoreBalance.getData().data.currencyCode,
            this.rustCoreBalance.getData().data.available
        )
        this.signature = rustCoreBalance.getData().signature
    }

    constructor(rustCoreBalance: RustCoreBalance) {
        this.rustCoreBalance = rustCoreBalance

        this.metadata = this.rustCoreBalance.getData().metadata
        this.data = this.rustCoreBalance.getData().data
        this.signature = this.rustCoreBalance.getData().signature
    }

    fun toJson(): String {
        return this.rustCoreBalance.toJsonString()
    }
}