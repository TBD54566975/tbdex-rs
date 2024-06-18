# tbDEX API Design (APID) <!-- omit in toc -->

**Last Updated:** June 13, 2024

**Custom DSL Version:** 0.1.0

- [Resources](#resources)
  - [`ResourceKind`](#resourcekind)
  - [`Resource`](#resource)
  - [`ResourceMetadata`](#resourcemetadata)
  - [`Offering`](#offering)
    - [`OfferingData`](#offeringdata)
      - [`PayinDetails`](#payindetails)
        - [`PayinMethod`](#payinmethod)
      - [`PayoutDetails`](#payoutdetails)
        - [`PayoutMethod`](#payoutmethod)
  - [`Balance`](#balance)
    - [`BalanceData`](#balancedata)
- [Messages](#messages)
  - [`MessageKind`](#messagekind)
  - [`Message`](#message)
  - [`MessageMetadata`](#messagemetadata)
  - [`Rfq`](#rfq)
    - [`CreateRfqData`](#createrfqdata)
      - [`CreateSelectedPayinMethod`](#createselectedpayinmethod)
      - [`CreateSelectedPayoutMethod`](#createselectedpayoutmethod)
    - [`RfqData`](#rfqdata)
      - [`SelectedPayinMethod`](#selectedpayinmethod)
      - [`SelectedPayoutMethod`](#selectedpayoutmethod)
    - [`RfqPrivateData`](#rfqprivatedata)
      - [`PrivatePaymentDetails`](#privatepaymentdetails)
  - [`Quote`](#quote)
    - [`QuoteData`](#quotedata)
      - [`QuoteDetails`](#quotedetails)
        - [`PaymentInstructions`](#paymentinstructions)
  - [`Order`](#order)
  - [`OrderStatus`](#orderstatus)
    - [`OrderStatusData`](#orderstatusdata)
  - [`Close`](#close)
    - [`CloseData`](#closedata)

🚧 Codify the Web5 APID dependencies 🚧

🚧 "valid next" 🚧

# Resources

## `ResourceKind`

```pseudocode!
ENUM ResourceKind
  offering,
  balance,
```

## `Resource`

```pseudocode!
INTERFACE Resource
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

> 🚧 `signature` is a required field on all `Resource` implementations, but we have a `sign()` method which must be called
> 
> 🚧 instead we should've passed a `Signer` (or `BearerDid`) into the constructor

## `ResourceMetadata`

```pseudocode!
CLASS ResourceMetadata
  PUBLIC DATA kind: string
  PUBLIC DATA from: string
  PUBLIC DATA to: string
  PUBLIC DATA id: string
  PUBLIC DATA protocol: string
  PUBLIC DATA createdAt: string
  PUBLIC DATA updatedAt: string?
```

## `Offering`

```pseudocode!
CLASS Offering IMPLEMENTS Resource
  PUBLIC DATA metadata: ResourceMetadata
  PUBLIC DATA data: OfferingData
  PUBLIC DATA signature: string
  CONSTRUCTOR(from: string, data: OfferingData, protocol: string)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `OfferingData`

```pseudocode!
CLASS OfferingData
  PUBLIC DATA description: string
  PUBLIC DATA payoutUnitsPerPayinUnit: string
  PUBLIC DATA payin PayinDetails
  PUBLIC DATA payout PayoutDetails
  PUBLIC DATA requiredClaims PresentationDefinition
```

#### `PayinDetails`

```pseudocode!
CLASS PayinDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
  PUBLIC DATA methods: []PayinMethod
```

##### `PayinMethod`

```pseudocode!
CLASS PayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA name: string?
  PUBLIC DATA description: string?
  PUBLIC DATA group: string?
  PUBLIC DATA requiredPaymentDetails: JsonNode? // 🚧
  PUBLIC DATA fee: string?
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
```

#### `PayoutDetails`

```pseudocode!
CLASS PayoutDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
  PUBLIC DATA methods: []PayoutMethod
```

##### `PayoutMethod`

```pseudocode!
CLASS PayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA name: string?
  PUBLIC DATA description: string?
  PUBLIC DATA group: string?
  PUBLIC DATA requiredPaymentDetails: JsonNode? // 🚧
  PUBLIC DATA fee: string?
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
  PUBLIC DATA estimatedSettlementTime: int
```

## `Balance`

```pseudocode!
CLASS Balance IMPLEMENTS Resource
  PUBLIC DATA metadata: ResourceMetadata
  PUBLIC DATA data: BalanceData
  PUBLIC DATA signature: string
  CONSTRUCTOR(from: string, data: BalanceData, protocol: string)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `BalanceData`

```pseudocode!
CLASS BalanceData
  PUBLIC DATA currencyCode: string
  PUBLIC DATA available: string
```

# Messages

## `MessageKind`

```pseudocode!
ENUM MessageKind
  rfq,
  quote,
  order,
  orderstatus,
  close,
```

## `Message`

```pseudocode!
INTERFACE Message
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

## `MessageMetadata`

```pseudocode!
CLASS MessageMetadata
  PUBLIC DATA from: string
  PUBLIC DATA to: string
  PUBLIC DATA kind: MessageKind
  PUBLIC DATA id: string
  PUBLIC DATA exchangeId: string
  PUBLIC DATA externalId: string?
  PUBLIC DATA createdAt: string
  PUBLIC DATA protocol: string
```

## `Rfq`

```pseudocode!
CLASS Rfq IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: RfqData
  PUBLIC DATA privateData: RfqPrivateData
  PUBLIC DATA signature: string
  CONSTRUCTOR(to: string, from: string, rfqData: CreateRfqData, protocol: string, externalId: string?)
  CONSTRUCTOR(json: string, requireAllPrivateData: bool?)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
  METHOD verifyOfferingRequirements(offering: Offering): bool
  METHOD verifyAllPrivateData(): bool
  METHOD verifyPresentPrivateData(): bool
```

### `CreateRfqData`

```pseudocode!
CLASS CreateRfqData
  PUBLIC DATA offeringId: string
  PUBLIC DATA payin: CreateSelectedPayinMethod
  PUBLIC DATA payout: CreateSelectedPayoutMethod
  PUBLIC DATA claims: string[]
```

#### `CreateSelectedPayinMethod`

```pseudocode!
CLASS CreateSelectedPayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetails: Map<string, JsonNode>? // 🚧
  PUBLIC DATA amount: string
```

#### `CreateSelectedPayoutMethod`

```pseudocode!
CLASS CreateSelectedPayoutMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetails: Map<string, JsonNode> // 🚧
```

### `RfqData`

```pseudocode!
CLASS RfqData
  PUBLIC DATA offeringId: string
  PUBLIC DATA payin: SelectedPayinMethod
  PUBLIC DATA payout: SelectedPayoutMethod
  PUBLIC DATA claimsHash: string?
```

#### `SelectedPayinMethod`

```pseudocode!
CLASS SelectedPayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetailsHash: string?
  PUBLIC DATA amount: string
```

#### `SelectedPayoutMethod`

```pseudocode!
CLASS SelectedPayoutMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetailsHash: string?
```

### `RfqPrivateData`

```pseudocode!
CLASS RfqPrivateData
  PUBLIC DATA salt: string
  PUBLIC DATA payin: PrivatePaymentDetails?
  PUBLIC DATA payout: PrivatePaymentDetails?
  PUBLIC DATA claims: string[]?
```

#### `PrivatePaymentDetails`

```pseudocode!
CLASS PrivatePaymentDetails
  PUBLIC DATA paymentDetails: Map<string, JsonNode>? // 🚧
```

## `Quote`

```pseudocode!
CLASS Quote IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: QuoteData
  PUBLIC DATA signature: string
  CONSTRUCTOR(to: string, from: string, exchangeId: string, quoteData: QuoteData, protocol: string, externalId: string?)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `QuoteData`

```pseudocode!
CLASS QuoteData
  PUBLIC DATA expiresAt: string
  PUBLIC DATA payin: QuoteDetails
  PUBLIC DATA payout: QuoteDetails
```

#### `QuoteDetails`

```pseudocode!
CLASS QuoteDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA amount: string
  PUBLIC DATA fee: string?
  PUBLIC DATA paymentInstructions: PaymentInstructions?
```

##### `PaymentInstructions`

```pseudocode!
CLASS PaymentInstructions
  PUBLIC DATA link: string?
  PUBLIC DATA instruction: string?
```

## `Order`

```pseudocode!
CLASS Order IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA signature: string
  CONSTRUCTOR(to: string, from: string, exchangeId: string, protocol: string, externalId: string?)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

## `OrderStatus`

```pseudocode!
CLASS OrderStatus IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: OrderStatusData
  PUBLIC DATA signature: string
  CONSTRUCTOR(to: string, from: string, exchangeId: string, orderStatusData: OrderStatusData, protocol: string, externalId: string?)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `OrderStatusData`

```pseudocode!
CLASS OrderStatusData
  PUBLIC DATA orderStatus: string
```

## `Close`

```pseudocode!
CLASS Close IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: CloseData
  PUBLIC DATA signature: string
  CONSTRUCTOR(to: string, from: string, exchangeId: string, closeData: CloseData, protocol: string, externalId: string?)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `CloseData`

```pseudocode!
CLASS CloseData
  PUBLIC DATA reason: string?
  PUBLIC DATA success: bool?
```