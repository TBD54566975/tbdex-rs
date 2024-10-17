import { Offering, BearerDid } from 'tbdex';

export class OfferingRepository {
  private offerings: Offering[] = [];

  constructor(pfiBearerDid: BearerDid) {
    const payinDetails = {
      currencyCode: "USD",
      methods: [
        {
          kind: "USD_LEDGER"
        }
      ]
    };

    const payoutDetails = {
      currencyCode: "KES",
      methods: [
        {
          kind: "MOMO_MPESA",
          requiredPaymentDetails: {
            "$schema": "http://json-schema.org/draft-07/schema#",
            title: "Mobile Money Required Payment Details",
            type: "object",
            required: ["phoneNumber", "reason"],
            additionalProperties: false,
            properties: {
              phoneNumber: {
                title: "Mobile money phone number",
                description: "Phone number of the Mobile Money account",
                type: "string"
              },
              reason: {
                title: "Reason for sending",
                description: "To abide by the travel rules and financial reporting requirements, the reason for sending money",
                type: "string"
              }
            }
          },
          estimatedSettlementTime: 10
        }
      ]
    };

    const requiredClaims = {
      id: "7ce4004c-3c38-4853-968b-e411bafcd945",
      input_descriptors: [
        {
          id: "bbdb9b7c-5754-4f46-b63b-590bada959e0",
          constraints: {
            fields: [
              {
                path: ["$.issuer"],
                filter: {
                  type: "string",
                  const: pfiBearerDid.did.uri
                },
                optional: false
              }
            ]
          }
        }
      ]
    };

    const cancellationDetails = {
      enabled: false
    };

    const offering = Offering.create(
      pfiBearerDid.did.uri,
      {
        description: "fake offering 1",
        payoutUnitsPerPayinUnit: "0.0069",
        payin: payinDetails,
        payout: payoutDetails,
        requiredClaims: requiredClaims,
        cancellation: cancellationDetails
      }
    );

    this.offerings.push(offering);
  }

  async signAndVerifyOfferings(bearerDid) {
    for(let offering of this.offerings) {
      await offering.sign(bearerDid)
      await offering.verify();
      console.log("offering signed and verified, ready for use")
    }
  }

  getOffering(offeringId: string): Offering {
    const offering = this.offerings.find(o => o.metadata.id === offeringId);
    if (!offering) {
      throw new Error(`No offering with ID ${offeringId}`);
    }
    return offering;
  }

  getOfferings(): Offering[] {
    return this.offerings;
  }
}
