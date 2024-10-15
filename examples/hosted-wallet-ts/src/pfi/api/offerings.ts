import { Router } from 'express';
import { GetOfferingsResponseBody } from 'tbdex';
import { OfferingRepository } from '../data/offering-repository';

// import "./of"

export default function offeringsRouter(offeringsRepository: OfferingRepository) {
  const router = Router();

  router.get('/', (req, res) => {
    console.log('GET /offerings');

    const offerings = offeringsRepository.getOfferings();
    const responseBody = new GetOfferingsResponseBody(offerings);

    res.type('application/json');
    res.send(responseBody.toJSONString());
  });

  return router;
}



// import { Router } from 'express';
// import {
//   Offering,
//   OfferingData,
//   PayinDetails,
//   PayinMethod,
//   PayoutDetails,
//   PayoutMethod,
//   PresentationDefinition,
//   InputDescriptor,
//   Constraints,
//   Field,
//   Filter,
//   CancellationDetails,
//   BearerDid,
//   PortableDid,
//   GetOfferingsResponseBody
// } from 'tbdex';

// const portableDID: PortableDid = JSON.parse(
//   '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
// );

// const bearerDid = BearerDid.fromPortableDID(portableDID);

// class OfferingRepository {
//   private offerings: Offering[] = [];

//   constructor(private bearerDid: BearerDid) {
//     // Seed the fake data for offerings, similar to the Kotlin example
//     const payinDetails = {
//       currencyCode: "USD",
//       methods: [
//         {
//           kind: "USD_LEDGER"
//         }
//       ]
//     };
    
//     const payoutDetails = {
//       currencyCode: "KES",
//       methods: [
//         {
//           kind: "MOMO_MPESA",
//           requiredPaymentDetails: {
//             "$schema": "http://json-schema.org/draft-07/schema#",
//             title: "Mobile Money Required Payment Details",
//             type: "object",
//             required: ["phoneNumber", "reason"],
//             additionalProperties: false,
//             properties: {
//               phoneNumber: {
//                 title: "Mobile money phone number",
//                 description: "Phone number of the Mobile Money account",
//                 type: "string"
//               },
//               reason: {
//                 title: "Reason for sending",
//                 description: "To abide by the travel rules and financial reporting requirements, the reason for sending money",
//                 type: "string"
//               }
//             }
//           },
//           estimatedSettlementTime: 10
//         }
//       ]
//     };
    
//     const requiredClaims = {
//       id: "7ce4004c-3c38-4853-968b-e411bafcd945",
//       input_descriptors: [
//         {
//           id: "bbdb9b7c-5754-4f46-b63b-590bada959e0",
//           constraints: {
//             fields: [
//               {
//                 path: ["$.issuer"],
//                 filter: {
//                   type: "string",
//                   const: "did:example:123"
//                 },
//                 optional: false
//               }
//             ]
//           }
//         }
//       ]
//     };
    
//     const cancellationDetails = {
//       enabled: false
//     };
    
//     // Use Offering.create() with plain JS objects for inner structures
//     const offering = Offering.create(
//       "did:example:123", // From field (Bearer DID URI)
//       {
//         description: "fake offering 1",
//         payoutUnitsPerPayinUnit: "0.0069",
//         payin: payinDetails,
//         payout: payoutDetails,
//         requiredClaims: requiredClaims,
//         cancellation: cancellationDetails
//       }
//     );

//     offering.sign(this.bearerDid);
//     offering.verify();

//     this.offerings.push(offering);
//   }

//   getOffering(offeringId: string): Offering {
//     const offering = this.offerings.find(o => o.metadata.id === offeringId);
//     if (!offering) {
//       throw new Error(`No offering with ID ${offeringId}`);
//     }
//     return offering;
//   }

//   getOfferings(): Offering[] {
//     return this.offerings;
//   }
// }

// const offeringsRepository = new OfferingRepository(bearerDid);

// // Express router setup
// const router = Router();

// router.get('/', (req, res) => {
//   console.log('GET /offerings');

//   const offerings = offeringsRepository.getOfferings();
//   const responseBody = new GetOfferingsResponseBody(offerings);

//   res.type('application/json');
//   res.send(responseBody.toJSONString());
// });

// export default router;