package tbdex.sdk.protocol.models

import com.fasterxml.jackson.databind.JsonNode
import tbdex.sdk.protocol.serialization.Json
import tbdex.sdk.rust.PayinMethodData as RustCorePayinMethodData

data class PayinMethod(
    val kind: String,
    val name: String? = null,
    val description: String? = null,
    val group: String? = null,
    val requiredPaymentDetails: JsonNode? = null,
    val fee: String? = null,
    val min: String? = null,
    val max: String? = null,
) {
    internal fun toRustCore(): RustCorePayinMethodData {
        return RustCorePayinMethodData(
            kind = this.kind,
            name = this.name,
            description = this.description,
            group = this.group,
            requiredPaymentDetails = this.requiredPaymentDetails?.let { Json.stringify(it) } ?: "",
            fee = this.fee,
            min = this.min,
            max = this.max,
        )
    }
}