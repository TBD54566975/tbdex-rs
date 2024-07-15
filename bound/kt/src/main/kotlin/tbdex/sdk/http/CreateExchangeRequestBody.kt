package tbdex.sdk.http

import tbdex.sdk.messages.Rfq
import tbdex.sdk.rust.CreateExchangeRequestBody as RustCoreCreateExchangeRequestBody
import tbdex.sdk.rust.SystemArchitecture

class CreateExchangeRequestBody {
    val message: Rfq
    val replyTo: String?

    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    constructor(json: String) {
        val rustCoreCreateExchangeRequestBody = RustCoreCreateExchangeRequestBody.fromJsonString(json)
        val data = rustCoreCreateExchangeRequestBody.getData()
        this.message = Rfq(data.message)
        this.replyTo = data.replyTo
    }
}