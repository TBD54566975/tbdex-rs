package tbdex.sdk.httpclient.request

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.HttpRequestBody as RustCoreHttpRequestBody

class Body {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    val message: Message
    val replyTo: String?

    private val rustCoreHttpRequestBody: RustCoreHttpRequestBody

    constructor(message: Message, replyTo: String? = null) {
        this.rustCoreHttpRequestBody = RustCoreHttpRequestBody(message.toJson(), replyTo)

        val data = this.rustCoreHttpRequestBody.getData()
        this.message = Message.fromJsonString(data.kind, data.jsonSerializedMessage)
        this.replyTo = data.replyTo
    }

    fun toJson(): String {
        return this.rustCoreHttpRequestBody.toJson()
    }

    constructor(json: String) {
        this.rustCoreHttpRequestBody = RustCoreHttpRequestBody.fromJsonString(json)

        val data = this.rustCoreHttpRequestBody.getData()
        this.message = Message.fromJsonString(data.kind, data.jsonSerializedMessage)
        this.replyTo = data.replyTo
    }
}