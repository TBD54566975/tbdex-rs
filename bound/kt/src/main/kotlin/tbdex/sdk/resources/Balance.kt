package tbdex.sdk.resources

import tbdex.sdk.web5.BearerDid
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData

typealias BalanceData = RustCoreBalanceData

class Balance {
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

    fun toJson(): String {
        return this.rustCoreBalance.toJson()
    }
}