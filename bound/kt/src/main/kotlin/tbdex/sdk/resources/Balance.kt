package tbdex.sdk.resources

import tbdex.sdk.TbdexException
import tbdex.sdk.rust.Balance as RustCoreBalance
import tbdex.sdk.rust.BalanceDataData as RustCoreBalanceData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

/**
 * Represents the data for a Balance resource in the tbDEX protocol.
 *
 * @property currencyCode The ISO 4217 currency code for the balance.
 * @property available The available amount of the specified currency.
 */
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

/**
 * Represents a Balance resource in the tbDEX protocol.
 *
 * A Balance resource communicates the amounts of each currency held by a PFI on behalf of a customer.
 *
 * @property metadata Metadata about the resource, including the sender and resource type.
 * @property data The balance data, including currency code and available balance.
 * @property signature The signature verifying the authenticity and integrity of the Balance resource.
 * @property rustCoreBalance The underlying RustCore representation of the Balance resource.
 */
data class Balance private constructor(
    val metadata: ResourceMetadata,
    val data: BalanceData,
    val signature: String,
    internal val rustCoreBalance: RustCoreBalance
) {
    companion object {
        /**
         * Creates a new Balance resource.
         *
         * @param from The DID of the sender (the PFI).
         * @param data The balance data containing the currency code and available balance.
         * @param protocol Optional protocol version.
         * @return The newly created Balance resource.
         * @throws TbdexException if the creation process fails.
         */
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

        /**
         * Parses a Balance resource from a JSON string.
         *
         * @param json The JSON string representing the Balance resource.
         * @return The deserialized Balance resource.
         * @throws TbdexException if parsing fails.
         */
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

    /**
     * Serializes the Balance resource to a JSON string.
     *
     * @return The serialized JSON string of the Balance resource.
     * @throws TbdexException if serialization fails.
     */
    fun toJsonString(): String {
        try {
            return rustCoreBalance.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Signs the Balance resource using the provided Bearer DID.
     *
     * @param bearerDid The Bearer DID used to sign the Balance resource.
     * @throws TbdexException if the signing process fails.
     */
    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreBalance.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    /**
     * Verifies the Balance resource's signature and validity.
     *
     * @throws TbdexException if verification fails.
     */
    fun verify() {
        try {
            rustCoreBalance.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}