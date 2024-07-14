# tbDEX API Design (APID) <!-- omit in toc -->

**Last Updated:** June 13, 2024

**Custom DSL Version:** 0.1.0

- [Web5 Dependencies](#web5-dependencies)
- [`JsonString`](#jsonstring)
- [`Signed`](#signed)
- [Resources](#resources)
  - [`ResourceKind`](#resourcekind)
  - [`ResourceMetadata`](#resourcemetadata)
  - [`Resource`](#resource)
  - [`Offering`](#offering)
    - [`OfferingData`](#offeringdata)
    - [`PayinDetails`](#payindetails)
    - [`PayinMethod`](#payinmethod)
    - [`PayoutDetails`](#payoutdetails)
    - [`PayoutMethod`](#payoutmethod)
    - [`CancellationDetails`](#cancellationdetails)
  - [`Balance`](#balance)
    - [`BalanceData`](#balancedata)
- [Messages](#messages)
  - [`MessageKind`](#messagekind)
  - [`MessageMetadata`](#messagemetadata)
  - [`Message`](#message)
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
    - [`PaymentInstruction`](#paymentinstruction)
  - [`Order`](#order)
  - [`Cancel`](#cancel)
    - [`CancelData`](#canceldata)
  - [`OrderStatus`](#orderstatus)
    - [`OrderStatusData`](#orderstatusdata)
  - [`Close`](#close)
    - [`CloseData`](#closedata)
- [HTTP](#http)
  - [`Exchange`](#exchange)
  - [`get_offerings()`](#get_offerings)
  - [`get_balances()`](#get_balances)
  - [`create_exchange()`](#create_exchange)
  - [`submit_order()`](#submit_order)
  - [`submit_close()`](#submit_close)
  - [`get_exchange()`](#get_exchange)
  - [`get_exchanges()`](#get_exchanges)

> [!WARNING]
>
> We need to define `JsonNode`

> [!WARNING]
>
> Snake vs camel casing is inconsistent

> [!WARNING]
>
> `FUNCTION` needs to be added to the Custom DSL

# Web5 Dependencies

- `PresentationDefinition`
- `BearerDid`

> [!WARNING]
>
> 🚧 Add links to Web5 APID

# `JsonString`

```pseudocode!
INTERFACE JsonString
  CONSTRUCTOR from_json_string(json: string): Self
  METHOD to_json_string(): string
```

# `Signed`

```pseudocode!
INTERFACE Signed
  PUBLIC DATA signature: string
  METHOD verify_signature(): Error?
```

# Resources

## `ResourceKind`

```pseudocode!
ENUM ResourceKind
  offering,
  balance,
```

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

## `Resource`

```pseudocode!
INTERFACE Resource
  PUBLIC DATA metadata: ResourceMetadata
```

## `Offering`

```pseudocode!
CLASS Offering IMPLEMENTS JsonString, Signed, Resource
  PUBLIC DATA data: OfferingData
  CONSTRUCTOR create(bearer_did: BearerDid, from: string, data: OfferingData, protocol: string)
```

### `OfferingData`

```pseudocode!
CLASS OfferingData
  PUBLIC DATA description: string
  PUBLIC DATA payoutUnitsPerPayinUnit: string
  PUBLIC DATA payin: PayinDetails
  PUBLIC DATA payout: PayoutDetails
  PUBLIC DATA requiredClaims: PresentationDefinition?
  PUBLIC DATA cancellation: CancellationDetails
```

### `PayinDetails`

```pseudocode!
CLASS PayinDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
  PUBLIC DATA methods: []PayinMethod
```

### `PayinMethod`

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

### `PayoutDetails`

```pseudocode!
CLASS PayoutDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA min: string?
  PUBLIC DATA max: string?
  PUBLIC DATA methods: []PayoutMethod
```

### `PayoutMethod`

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

### `CancellationDetails`

```pseudocode!
CLASS PayinMethod
  PUBLIC DATA enabled: bool
  PUBLIC DATA termsUrl: string?
  PUBLIC DATA terms: string?
```

## `Balance`

```pseudocode!
CLASS Balance IMPLEMENTS JsonString, Signed, Resource
  PUBLIC DATA data: BalanceData
  CONSTRUCTOR create(bearer_did: BearerDid, from: string, data: BalanceData, protocol: string)
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
  cancel,
  orderstatus,
  close,
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

## `Message`

```pseudocode!
INTERFACE Message
  PUBLIC DATA metadata: MessageMetadata
```

## `Rfq`

```pseudocode!
CLASS Rfq IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: RfqData
  PUBLIC DATA privateData: RfqPrivateData
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, rfqData: CreateRfqData, protocol: string, externalId: string?)
```

### `CreateRfqData`

```pseudocode!
CLASS CreateRfqData
  PUBLIC DATA offeringId: string
  PUBLIC DATA payin: CreateSelectedPayinMethod
  PUBLIC DATA payout: CreateSelectedPayoutMethod
  PUBLIC DATA claims: []string
```

### `CreateSelectedPayinMethod`

```pseudocode!
CLASS CreateSelectedPayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetails: Map<string, JsonNode>? // 🚧
  PUBLIC DATA amount: string
```

### `CreateSelectedPayoutMethod`

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

### `SelectedPayinMethod`

```pseudocode!
CLASS SelectedPayinMethod
  PUBLIC DATA kind: string
  PUBLIC DATA paymentDetailsHash: string?
  PUBLIC DATA amount: string
```

### `SelectedPayoutMethod`

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
  PUBLIC DATA claims: []string?
```

### `PrivatePaymentDetails`

```pseudocode!
CLASS PrivatePaymentDetails
  PUBLIC DATA paymentDetails: Map<string, JsonNode>? // 🚧
```

## `Quote`

```pseudocode!
CLASS Quote IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: QuoteData
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, exchangeId: string, quoteData: QuoteData, protocol: string, externalId: string?)
```

### `QuoteData`

```pseudocode!
CLASS QuoteData
  PUBLIC DATA expiresAt: string
  PUBLIC DATA payin: QuoteDetails
  PUBLIC DATA payout: QuoteDetails
```

### `QuoteDetails`

```pseudocode!
CLASS QuoteDetails
  PUBLIC DATA currencyCode: string
  PUBLIC DATA subtotal: string
  PUBLIC DATA total: string
  PUBLIC DATA fee: string?
  PUBLIC DATA paymentInstruction: PaymentInstruction?
```

### `PaymentInstruction`

```pseudocode!
CLASS PaymentInstruction
  PUBLIC DATA link: string?
  PUBLIC DATA instruction: string?
```

## `Order`

```pseudocode!
CLASS Order IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: Empty
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, exchangeId: string, protocol: string, externalId: string?)
```

## `Cancel`

```pseudocode!
CLASS Cancel IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: CancelData
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, exchangeId: string, cancelData: CancelData, protocol: string, externalId: string?)
```

### `CancelData`

```pseudocode!
CLASS CancelData
  PUBLIC DATA reason: string
```

## `OrderStatus`

```pseudocode!
CLASS OrderStatus IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: OrderStatusData
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, exchangeId: string, orderStatusData: OrderStatusData, protocol: string, externalId: string?)
```

### `OrderStatusData`

```pseudocode!
CLASS OrderStatusData
  PUBLIC DATA orderStatus: string
```

## `Close`

```pseudocode!
CLASS Close IMPLEMENTS JsonString, Signed, Message
  PUBLIC DATA data: CloseData
  CONSTRUCTOR(bearer_did: BearerDid, to: string, from: string, exchangeId: string, closeData: CloseData, protocol: string, externalId: string?)
```

### `CloseData`

```pseudocode!
CLASS CloseData
  PUBLIC DATA reason: string?
  PUBLIC DATA success: bool?
```

# HTTP

## `Exchange`

```pseudocode!
CLASS Exchange
  PUBLIC DATA rfq: Rfq
  PUBLIC DATA quote: Quote
  PUBLIC DATA order: Order
  PUBLIC DATA order_statuses: []OrderStatus
  PUBLIC DATA close: Close
```

## `get_offerings()`

```pseudocode!
FUNCTION get_offerings(pfi_did_uri: string): []Offering
```

## `get_balances()`

```pseudocode!
FUNCTION get_balances(pfi_did_uri: string, bearer_did: BearerDid): []Balance
```

## `create_exchange()`

```pseudocode!
FUNCTION create_exchange(rfq: Rfq, reply_to: string?)
```

## `submit_order()`

```pseudocode!
FUNCTION submit_order(order: Order)
```

## `submit_close()`

```pseudocode!
FUNCTION submit_close(order: Order)
```

## `get_exchange()`

```pseudocode!
FUNCTION get_exchange(pfi_did_uri: string, bearer_did: BearerDid, exchange_id: string): Exchange
```

## `get_exchanges()`

```pseudocode!
FUNCTION get_exchanges(pfi_did_uri: string, bearer_did: BearerDid): []string
```
