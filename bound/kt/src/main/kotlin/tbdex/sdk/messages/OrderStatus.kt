package tbdex.sdk.messages

import tbdex.sdk.TbdexException
import tbdex.sdk.http.ReplyToMessage
import tbdex.sdk.rust.OrderStatusStatus as RustCoreStatus
import tbdex.sdk.rust.OrderStatus as RustCoreOrderStatus
import tbdex.sdk.rust.OrderStatusDataData as RustCoreOrderStatusData
import tbdex.sdk.rust.fromWeb5
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid

enum class Status {
    PAYIN_PENDING,
    PAYIN_INITIATED,
    PAYIN_SETTLED,
    PAYIN_FAILED,
    PAYIN_EXPIRED,
    PAYOUT_PENDING,
    PAYOUT_INITIATED,
    PAYOUT_SETTLED,
    PAYOUT_FAILED,
    REFUND_PENDING,
    REFUND_INITIATED,
    REFUND_SETTLED,
    REFUND_FAILED;

    companion object {
        internal fun fromRustCore(rustCore: RustCoreStatus): Status {
            return when (rustCore) {
                RustCoreStatus.PAYIN_PENDING -> PAYIN_PENDING
                RustCoreStatus.PAYIN_INITIATED -> PAYIN_INITIATED
                RustCoreStatus.PAYIN_SETTLED -> PAYIN_SETTLED
                RustCoreStatus.PAYIN_FAILED -> PAYIN_FAILED
                RustCoreStatus.PAYIN_EXPIRED -> PAYIN_EXPIRED
                RustCoreStatus.PAYOUT_PENDING -> PAYOUT_PENDING
                RustCoreStatus.PAYOUT_INITIATED -> PAYOUT_INITIATED
                RustCoreStatus.PAYOUT_SETTLED -> PAYOUT_SETTLED
                RustCoreStatus.PAYOUT_FAILED -> PAYOUT_FAILED
                RustCoreStatus.REFUND_PENDING -> REFUND_PENDING
                RustCoreStatus.REFUND_INITIATED -> REFUND_INITIATED
                RustCoreStatus.REFUND_SETTLED -> REFUND_SETTLED
                RustCoreStatus.REFUND_FAILED -> REFUND_FAILED
            }
        }
    }

    internal fun toRustCore(): RustCoreStatus {
        return when (this) {
            PAYIN_PENDING -> RustCoreStatus.PAYIN_PENDING
            PAYIN_INITIATED -> RustCoreStatus.PAYIN_INITIATED
            PAYIN_SETTLED -> RustCoreStatus.PAYIN_SETTLED
            PAYIN_FAILED -> RustCoreStatus.PAYIN_FAILED
            PAYIN_EXPIRED -> RustCoreStatus.PAYIN_EXPIRED
            PAYOUT_PENDING -> RustCoreStatus.PAYOUT_PENDING
            PAYOUT_INITIATED -> RustCoreStatus.PAYOUT_INITIATED
            PAYOUT_SETTLED -> RustCoreStatus.PAYOUT_SETTLED
            PAYOUT_FAILED -> RustCoreStatus.PAYOUT_FAILED
            REFUND_PENDING -> RustCoreStatus.REFUND_PENDING
            REFUND_INITIATED -> RustCoreStatus.REFUND_INITIATED
            REFUND_SETTLED -> RustCoreStatus.REFUND_SETTLED
            REFUND_FAILED -> RustCoreStatus.REFUND_FAILED
        }
    }
}

data class OrderStatusData (
    val status: Status,
    val details: String?
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreOrderStatusData): OrderStatusData {
            return OrderStatusData(Status.fromRustCore(rustCore.status), rustCore.details)
        }
    }

    internal fun toRustCore(): RustCoreOrderStatusData {
        return RustCoreOrderStatusData(status.toRustCore(), details)
    }
}

data class OrderStatus private constructor(
    val metadata: MessageMetadata,
    val data: OrderStatusData,
    val signature: String,
    internal val rustCoreOrderStatus: RustCoreOrderStatus
): Message, ReplyToMessage {
    companion object {
        fun create(
            to: String,
            from: String,
            exchangeId: String,
            data: OrderStatusData,
            protocol: String? = null,
            externalId: String? = null
        ): OrderStatus {
            try {
                val rustCoreOrderStatus = RustCoreOrderStatus.create(to, from, exchangeId, data.toRustCore(), protocol, externalId)
                val rustCoreData = rustCoreOrderStatus.getData()
                return OrderStatus(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    OrderStatusData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreOrderStatus
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        fun fromJsonString(json: String): OrderStatus {
            try {
                val rustCoreOrderStatus = RustCoreOrderStatus.fromJsonString(json)
                val rustCoreData = rustCoreOrderStatus.getData()
                return OrderStatus(
                    MessageMetadata.fromRustCore(rustCoreData.metadata),
                    OrderStatusData.fromRustCore(rustCoreData.data),
                    rustCoreData.signature,
                    rustCoreOrderStatus
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }

        internal fun fromRustCoreOrderStatus(rustCoreOrderStatus: RustCoreOrderStatus): OrderStatus {
            val rustCoreData = rustCoreOrderStatus.getData()
            return OrderStatus(
                MessageMetadata.fromRustCore(rustCoreData.metadata),
                OrderStatusData.fromRustCore(rustCoreData.data),
                rustCoreData.signature,
                rustCoreOrderStatus
            )
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreOrderStatus.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun sign(bearerDid: BearerDid) {
        try {
            rustCoreOrderStatus.sign(RustCoreBearerDid.fromWeb5(bearerDid))
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }

    fun verify() {
        try {
            rustCoreOrderStatus.verify()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}