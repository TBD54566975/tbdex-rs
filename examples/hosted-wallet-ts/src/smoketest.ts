import * as tbdex from "tbdex";

console.log("Starting smoke test!");

console.log(tbdex);

const portableDID: tbdex.PortableDid = JSON.parse(
  '{"uri":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","document":{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0","type":"JsonWebKey","controller":"did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}}],"authentication":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"assertionMethod":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityInvocation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"],"capabilityDelegation":["did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJpQXV4Q29hclJhaXpHMVpIMHphalRrcmJfUGstN3pNLXpGVzREQThBSzVNIn0#0"]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"GSd8aUVcNX9O8ipqOV2gXJToHyzUZ_8mJrQ7G5UsmHs","x":"iAuxCoarRaizG1ZH0zajTkrb_Pk-7zM-zFW4DA8AK5M"}]}'
);

console.log(portableDID);

const bearerDID = tbdex.BearerDid.fromPortableDID(portableDID);

console.log(bearerDID);

const OfferingVector = {
  description: "Offering parses from string",
  input:
    '{"metadata": {"from": "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6IjdzRDAzOXdITjVybzVhWUxvNjMxaW9aTzVSdjlRS242aGpHamRwZkhFMFkifQ","kind": "offering","id": "offering_01j2h97kkre7tanx9d4cj1zv6e","createdAt": "2024-07-11T16:15:14Z","updatedAt": "2024-07-11T16:15:14Z","protocol": "1.0"},"data": {"description": "USDC for USD","payoutUnitsPerPayinUnit": "1.0","payin": {"currencyCode": "USD","min": "0.1","max": "1000","methods": [{"kind": "DEBIT_CARD","requiredPaymentDetails": {"$schema": "http://json-schema.org/draft-07/schema#","type": "object","properties": {"cardNumber": {"type": "string","description": "The 16-digit debit card number","minLength": 16,"maxLength": 16},"expiryDate": {"type": "string","description": "The expiry date of the card in MM/YY format","pattern": "^(0[1-9]|1[0-2])\\\\/([0-9]{2})$"},"cardHolderName": {"type": "string","description": "Name of the cardholder as it appears on the card"},"cvv": {"type": "string","description": "The 3-digit CVV code","minLength": 3,"maxLength": 3}},"required": ["cardNumber","expiryDate","cardHolderName","cvv"],"additionalProperties": false}}]},"payout": {"currencyCode": "USDC","max": "5000","methods": [{"kind": "STORED_BALANCE","estimatedSettlementTime": 1200}]},"requiredClaims": {"id": "foo","name": "kyccredential","purpose": "To verify the identity of the user","input_descriptors": [{"id": "1","name": "KYC Information","purpose": "To verify the identity of the user","constraints": {"fields": [{"path": ["$.type[0]"],"filter": {"type": "string","pattern": "KYC"}}]}}]},"cancellation": {"enabled": false}},"signature": "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJamR6UkRBek9YZElUalZ5YnpWaFdVeHZOak14YVc5YVR6VlNkamxSUzI0MmFHcEhhbVJ3WmtoRk1Ga2lmUSMwIn0..M9yF4FtmfeTvmUyutp-k76WFDjuAfJ9fdKdY93Sg1G3KE8KCoFPCQIborK8H22MG2MYsHKZGExEzDWkwXkTbAg"}',
  output: {
    metadata: {
      from: "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6IjdzRDAzOXdITjVybzVhWUxvNjMxaW9aTzVSdjlRS242aGpHamRwZkhFMFkifQ",
      kind: "offering",
      id: "offering_01j2h97kkre7tanx9d4cj1zv6e",
      createdAt: "2024-07-11T16:15:14Z",
      updatedAt: "2024-07-11T16:15:14Z",
      protocol: "1.0",
    },
    data: {
      description: "USDC for USD",
      payoutUnitsPerPayinUnit: "1.0",
      payin: {
        currencyCode: "USD",
        min: "0.1",
        max: "1000",
        methods: [
          {
            kind: "DEBIT_CARD",
            requiredPaymentDetails: {
              $schema: "http://json-schema.org/draft-07/schema#",
              type: "object",
              properties: {
                cardNumber: {
                  type: "string",
                  description: "The 16-digit debit card number",
                  minLength: 16,
                  maxLength: 16,
                },
                expiryDate: {
                  type: "string",
                  description: "The expiry date of the card in MM/YY format",
                  pattern: "^(0[1-9]|1[0-2])\\/([0-9]{2})$",
                },
                cardHolderName: {
                  type: "string",
                  description:
                    "Name of the cardholder as it appears on the card",
                },
                cvv: {
                  type: "string",
                  description: "The 3-digit CVV code",
                  minLength: 3,
                  maxLength: 3,
                },
              },
              required: ["cardNumber", "expiryDate", "cardHolderName", "cvv"],
              additionalProperties: false,
            },
          },
        ],
      },
      payout: {
        currencyCode: "USDC",
        max: "5000",
        methods: [
          {
            kind: "STORED_BALANCE",
            estimatedSettlementTime: 1200,
          },
        ],
      },
      requiredClaims: {
        id: "foo",
        name: "kyccredential",
        purpose: "To verify the identity of the user",
        input_descriptors: [
          {
            id: "1",
            name: "KYC Information",
            purpose: "To verify the identity of the user",
            constraints: {
              fields: [
                {
                  path: ["$.type[0]"],
                  filter: {
                    type: "string",
                    pattern: "KYC",
                  },
                },
              ],
            },
          },
        ],
      },
      cancellation: {
        enabled: false,
      },
    },
    signature:
      "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJamR6UkRBek9YZElUalZ5YnpWaFdVeHZOak14YVc5YVR6VlNkamxSUzI0MmFHcEhhbVJ3WmtoRk1Ga2lmUSMwIn0..M9yF4FtmfeTvmUyutp-k76WFDjuAfJ9fdKdY93Sg1G3KE8KCoFPCQIborK8H22MG2MYsHKZGExEzDWkwXkTbAg",
  },
  error: false,
};

const input = OfferingVector.input;
const offering = tbdex.Offering.fromJSONString(input);
console.log(offering);

const offeringJSONString = offering.toJSONString();
console.log(offeringJSONString);
const offeringJSON = JSON.parse(offeringJSONString);
console.log(offeringJSON);

await offering.verify();

console.log("Finished smoke test!");