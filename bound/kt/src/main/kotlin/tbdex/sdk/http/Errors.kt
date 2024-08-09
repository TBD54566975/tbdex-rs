package tbdex.sdk.http

import tbdex.sdk.rust.ErrorDetailData as RustCoreErrorDetail
import tbdex.sdk.rust.ErrorResponseBody as RustCoreErrorResponseBody

typealias ErrorDetail = RustCoreErrorDetail

class ErrorResponseBody private constructor(
    val message: String,
    val details: List<ErrorDetail>?,
    internal val rustCoreErrorResponseBody: RustCoreErrorResponseBody
) {
    constructor(message: String, details: List<ErrorDetail>? = null) : this(
        message,
        details,
        RustCoreErrorResponseBody(message, details),
    )

    companion object {
        fun fromJsonString(json: String): ErrorResponseBody {
            val rustCoreErrorResponseBody = RustCoreErrorResponseBody.fromJsonString(json)
            val data = rustCoreErrorResponseBody.getData()
            return ErrorResponseBody(data.message, data.details, rustCoreErrorResponseBody)
        }
    }

    fun toJsonString(): String {
        return this.rustCoreErrorResponseBody.toJsonString()
    }
}