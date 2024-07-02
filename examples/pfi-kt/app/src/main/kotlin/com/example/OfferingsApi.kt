package com.example

import spark.Spark.get
import tbdex.sdk.resources.*
import tbdex.sdk.web5.*

class OfferingsApi(private val bearerDid: BearerDid) {
    val offering: Offering = Offering(
        bearerDid,
        bearerDid.did.uri,
        OfferingData(
            description = "fake offering 1",
            payoutUnitsPerPayinUnit = "0.0069",
            payin = PayinDetails(
                currencyCode = "USD",
                methods = listOf(
                    PayinMethod(
                        kind = "USD_LEDGER",
                    )
                )
            ),
            payout = PayoutDetails(
                currencyCode = "KES",
                methods = listOf(
                    PayoutMethod(
                        kind = "MOMO_MPESA",
                        requiredPaymentDetails = mapOf(
                            "\$schema" to "http://json-schema.org/draft-07/schema#",
                            "title" to "Mobile Money Required Payment Details",
                            "type" to "object",
                            "required" to listOf("phoneNumber", "reason"),
                            "additionalProperties" to false,
                            "properties" to mapOf(
                                "phoneNumber" to mapOf(
                                    "title" to "Mobile money phone number",
                                    "description" to "Phone number of the Mobile Money account",
                                    "type" to "string"
                                ),
                                "reason" to mapOf(
                                    "title" to "Reason for sending",
                                    "description" to "To abide by the travel rules and financial reporting requirements, the reason for sending money",
                                    "type" to "string"
                                )
                            )
                        ),
                        estimatedSettlementTime = 10
                    )
                )
            ),
            requiredClaims = PresentationDefinition(
                id = "7ce4004c-3c38-4853-968b-e411bafcd945",
                name = null,
                purpose = null,
                inputDescriptors = listOf(
                    InputDescriptor(
                        id = "bbdb9b7c-5754-4f46-b63b-590bada959e0",
                        constraints = Constraints(
                            fields = listOf(
                                // TODO rust core PresentationDefinition select_credentials() is failing to select on type
//                                Field(
//                                    path = listOf("$.type[*]"),
//                                    filter = Filter(
//                                        type = "string",
//                                        pattern = "^SanctionCredential$"
//                                    ),
//                                    optional = false,
//                                ),
                                Field(
                                    path = listOf("$.issuer"),
                                    filter = Filter(
                                        type = "string",
                                        const = bearerDid.did.uri
                                    ),
                                    optional = false,
                                )
                            )
                        )
                    )
                )
            )
        ),
        "1.0"
    )

    fun setupGetOfferings() {
        get("/offerings") { _, res ->
            res.type("application/json")
            "{\"data\": [${offering.toJson()}]}"
        }
    }
}