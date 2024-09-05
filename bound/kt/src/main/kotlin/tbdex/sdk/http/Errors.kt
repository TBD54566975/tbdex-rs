package tbdex.sdk.http

import tbdex.sdk.TbdexException
import tbdex.sdk.rust.ErrorDetailData as RustCoreErrorDetail
import tbdex.sdk.rust.ErrorResponseBody as RustCoreErrorResponseBody

data class ErrorDetail(
    val id: String? = null,
    val message: String? = null,
    val path: String? = null,
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreErrorDetail): ErrorDetail {
            return ErrorDetail(rustCore.id, rustCore.message, rustCore.path)
        }
    }

    internal fun toRustCore(): RustCoreErrorDetail {
        return RustCoreErrorDetail(id, message, path)
    }
}

data class ErrorResponseBody private constructor(
    val message: String,
    val details: List<ErrorDetail>?,
    internal val rustCoreErrorResponseBody: RustCoreErrorResponseBody
) {
    constructor(message: String, details: List<ErrorDetail>? = null) : this(
        message,
        details,
        try {
            RustCoreErrorResponseBody(message, details?.map { it.toRustCore() })
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    )

    companion object {
        fun fromJsonString(json: String): ErrorResponseBody {
            try {
                val rustCoreErrorResponseBody = RustCoreErrorResponseBody.fromJsonString(json)
                val data = rustCoreErrorResponseBody.getData()
                return ErrorResponseBody(
                    data.message,
                    data.details?.map { ErrorDetail.fromRustCore(it) },
                    rustCoreErrorResponseBody
                )
            } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
                throw TbdexException.fromRustCore(e)
            }
        }
    }

    fun toJsonString(): String {
        try {
            return rustCoreErrorResponseBody.toJsonString()
        } catch (e: tbdex.sdk.rust.TbdexException.Exception) {
            throw TbdexException.fromRustCore(e)
        }
    }
}