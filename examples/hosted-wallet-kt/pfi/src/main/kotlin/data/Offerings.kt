package data

import tbdex.sdk.resources.*
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PresentationDefinition
import tbdex.sdk.web5.InputDescriptor
import tbdex.sdk.web5.Constraints
import tbdex.sdk.web5.Field
import tbdex.sdk.web5.Filter

class Offerings(private val bearerDid: BearerDid) {
    private var offerings: MutableList<Offering> = mutableListOf()

    init {
        // this app currently always seeds the fake data
        // and doesn't support the ability for developers to dynamically add/remove offerings

        val offering = Offering.create(
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
                ),
                cancellation = CancellationDetails(
                    enabled = false
                )
            )
        )

        offering.sign(bearerDid)
        offering.verify()

        offerings.add(offering)
    }

    fun getOffering(offeringId: String): Offering {
        return offerings.find { it.metadata.id == offeringId }
            ?: throw Exception("No offering with ID $offeringId")
    }

    fun getOfferings(): List<Offering> {
        return offerings
    }
}