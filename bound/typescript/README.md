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

### Message Creation

```typescript
import { 
  Rfq,
  CreateRfqData,
} from '@tbdex/sdk';

const createRfqData: CreateRfqData = {
    claims: [verifiableCredential],
    offeringId,
    payin: {
    amount: '101',
    kind: 'USD_LEDGER',
    paymentDetails: null,
    },
    payout: {
    kind: 'MOMO_MPESA',
    paymentDetails: {
        phoneNumber: '867-5309',
        reason: 'cause',
    },
    },
};

const rfq = Rfq.create(pfiDidUri, bearerDid.did.uri, createRfqData);

await rfq.sign(bearerDid);
await rfq.verify();
```

### Message Parsing
```typescript

import { 
  Rfq,
} from '@tbdex/sdk';

const jsonMessage = "<SERIALIZED_MESSAGE>"
const rfq = Rfq.fromJSONString(jsonMessage);
```

### Http Client Create Exchange Flow
```typescript
import {
  Rfq,
  CreateRfqData,
  BearerDid,
  getOfferings,
  createExchange,
  getExchange,
  Exchange,
} from '@tbdex/sdk';

// Step 1: Fetch offerings
console.log('1. Fetching offerings...');
const offerings = await getOfferings(pfiDidUri);
if (!offerings || offerings.length === 0) {
    throw new Error('No offerings available.');
}

const offeringId = offerings[0].metadata.id;
console.log(`Successfully fetched offering ID: ${offeringId}\n`);

// Step 2: Create exchange (RFQ)
console.log('2. Creating exchange...');
const createRfqData: CreateRfqData = {
    claims: [verifiableCredential],
    offeringId,
    payin: {
    amount: '101',
    kind: 'USD_LEDGER',
    paymentDetails: null,
    },
    payout: {
    kind: 'PAYOUT_KIND',
    paymentDetails: {
        phoneNumber: '867-5309',
        reason: 'cause',
    },
    },
};

const rfq = Rfq.create(pfiDidUri, bearerDid.did.uri, createRfqData);
await rfq.sign(bearerDid);
await rfq.verify();
await createExchange(rfq);

const exchangeId = rfq.metadata.exchangeId;
console.log(`Created exchange with ID: ${exchangeId}\n`);
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