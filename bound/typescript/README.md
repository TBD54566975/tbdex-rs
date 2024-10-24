# tbDEX SDK

[![npm version](https://badge.fury.io/js/%40tbdex%2Fsdk.svg)](https://badge.fury.io/js/%40tbdex%2Fsdk)

A TypeScript SDK for interacting with the tbDEX protocol. This SDK is built on a Rust core with WebAssembly bindings, offering high performance, type safety, and interoperability. This library that can be used to create, parse, verify, and validate the tbDEX Messages and Resources defined in the [protocol specification](https://github.com/TBD54566975/tbdex/blob/main/README.md). This library also includes an HTTP client that can be used to send tbdex messages to PFIs.


## Features

- **Complete tbDEX Protocol Support**: Implements all tbDEX message types and resources
- **Type-Safe**: Full TypeScript support with comprehensive type definitions
- **Cross-Platform**: Works in Node.js and modern browsers

## Installation

```bash
npm install @tbdex/sdk
```

## Basic Usage

### Creating and Managing Exchanges

```typescript
import { 
  BearerDid,
  Rfq,
  CreateRfqData,
  getOfferings,
  createExchange,
  getExchange
} from 'tbdex';

// Create a new Bearer DID
const bearerDid = await BearerDid.create();

// Fetch available offerings
const offerings = await getOfferings('did:example:pfi123');
const offeringId = offerings[0].metadata.id;

// Create an RFQ (Request for Quote)
const rfqData: CreateRfqData = {
  offeringId,
  claims: ['credentialJwt'],
  payin: {
    amount: '100',
    kind: 'USD_LEDGER',
    paymentDetails: null
  },
  payout: {
    kind: 'PAYOUT_KIND',
    paymentDetails: {
      phoneNumber: '555-0123',
      reason: 'payment'
    }
  }
};

// Create and sign the RFQ
const rfq = Rfq.create('did:example:pfi123', bearerDid.did.uri, rfqData);
await rfq.sign(bearerDid);
await rfq.verify();

// Submit the RFQ to create an exchange
await createExchange(rfq);

// Get exchange details
const exchange = await getExchange(
  'did:example:pfi123',
  bearerDid,
  rfq.metadata.exchangeId
);
```

## Message Types

The SDK supports all tbDEX message types:

- **RFQ (Request for Quote)**: Initial request for exchange
- **Quote**: Response with exchange terms
- **Order**: Acceptance of quote terms
- **OrderInstructions**: Payment process details
- **OrderStatus**: Transaction status updates
- **Close**: Exchange completion notification
- **Cancel**: Exchange cancellation

## Resources

Supported tbDEX resources:

- **Offering**: Defines exchange terms and requirements
- **Balance**: Shows available currency amounts

## Error Handling

The SDK provides detailed error information for common scenarios:

```typescript
try {
  await rfq.verify();
} catch (error) {
  if (error instanceof TbdexError) {
    console.error('Verification failed:', error.message);
  }
}
```