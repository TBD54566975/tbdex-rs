# tbDEX API Design (APID) <!-- omit in toc -->

**Last Updated:** June 13, 2024

**Custom DSL Version:** 0.1.0

- [Web5 Dependencies](#web5-dependencies)
- [Resources](#resources)
  - [`ResourceKind`](#resourcekind)
  - [`ResourceMetadata`](#resourcemetadata)
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
  - [`WalletUpdateMessage`](#walletupdatemessage)
  - [`ReplyToMessage`](#replytomessage)
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
  - [`Order`](#order)
  - [`OrderInstructions`](#orderinstructions)
    - [`OrderInstructionsData](#orderinstructionsdata)
    - [`PaymentInstruction`](#paymentinstruction)
  - [`Cancel`](#cancel)
    - [`CancelData`](#canceldata)
  - [`OrderStatus`](#orderstatus)
    - [`OrderStatusData`](#orderstatusdata)
  - [`Close`](#close)
    - [`CloseData`](#closedata)
- [HTTP](#http)
  - [`ErrorResponseBody`](#errorresponsebody)
    - [`ErrorDetail`](#errordetail)
  - [`GetOfferingsResponseBody`](#getofferingsresponsebody)
  - [`GetBalancesResponseBody`](#getbalancesresponsebody)
  - [`GetExchangeResponseBody`](#getexchangeresponsebody)
  - [`GetExchangesResponseBody`](#getexchangesresponsebody)
  - [`CreateExchangeRequestBody`](#createexchangerequestbody)
  - [`UpdateExchangeRequestBody`](#updateexchangerequestbody)
  - [`ReplyToRequestBody`](#replytorequestbody)
- [HTTP Client](#http-client)
  - [`get_offerings()`](#get_offerings)
  - [`get_balances()`](#get_balances)
  - [`create_exchange()`](#create_exchange)
  - [`submit_order()`](#submit_order)
  - [`submit_close()`](#submit_close)
  - [`get_exchange()`](#get_exchange)
  - [`Exchange`](#exchange)
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

## `Offering`

```pseudocode!
CLASS Offering IMPLEMENTS Resource
  PUBLIC DATA metadata: ResourceMetadata
  PUBLIC DATA data: OfferingData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(from: string, data: OfferingData, protocol: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD verify(): Error?
  METHOD to_json_string(): string
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
CLASS Balance IMPLEMENTS Resource
  PUBLIC DATA metadata: ResourceMetadata
  PUBLIC DATA data: BalanceData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(from: string, data: BalanceData, protocol: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD verify(): Error?
  METHOD to_json_string(): string
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
  orderinstructions,
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
```

## `WalletUpdateMessage`

```pseudocode!
INTERFACE WalletUpdateMessage
```

## `ReplyToMessage`

```pseudocode!
INTERFACE ReplyToMessage
```

## `Rfq`

```pseudocode!
CLASS Rfq IMPLEMENTS Message
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: RfqData
  PUBLIC DATA privateData: RfqPrivateData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, rfqData: CreateRfqData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string, requireAllPrivateData: bool?)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
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
CLASS Quote IMPLEMENTS Message, ReplyToMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: QuoteData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, quoteData: QuoteData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
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
```

## `Order`

```pseudocode!
CLASS Order IMPLEMENTS Message, WalletUpdateMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
```

## `OrderInstructions`

```pseudocode!
CLASS OrderInstructions IMPLEMENTS Message, ReplyToMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: OrderInstructionsData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, orderInstructionsData: OrderInstructionsData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
```

### `OrderInstructionsData`

```pseudocode!
CLASS OrderInstructionsData
  PUBLIC DATA payin: PaymentInstruction
  PUBLIC DATA payout: PaymentInstruction
```

### `PaymentInstruction`

```pseudocode!
CLASS PaymentInstruction
  PUBLIC DATA link: string?
  PUBLIC DATA instruction: string?
```

## `Cancel`

```pseudocode!
CLASS Cancel IMPLEMENTS Message, WalletUpdateMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: CancelData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, cancelData: CancelData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
```

### `CancelData`

```pseudocode!
CLASS CancelData
  PUBLIC DATA reason: string
```

## `OrderStatus`

```pseudocode!
CLASS OrderStatus IMPLEMENTS Message, ReplyToMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: OrderStatusData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, orderStatusData: OrderStatusData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
```

### `OrderStatusData`

```pseudocode!
CLASS OrderStatusData
  PUBLIC DATA orderStatus: string
```

## `Close`

```pseudocode!
CLASS Close IMPLEMENTS Message, ReplyToMessage
  PUBLIC DATA metadata: MessageMetadata
  PUBLIC DATA data: CloseData
  PUBLIC DATA signature: string
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, closeData: CloseData, protocol: string?, externalId: string?)
  CONSTRUCTOR from_json_string(json: string)
  METHOD verify(): Error?
  METHOD sign(bearer_did: BearerDid): Error?
  METHOD to_json_string(): string
```

### `CloseData`

```pseudocode!
CLASS CloseData
  PUBLIC DATA reason: string?
  PUBLIC DATA success: bool?
```

# HTTP

## `ErrorResponseBody`

[Reference.](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#error-responses)

```pseudocode!
CLASS ErrorResponseBody
  PUBLIC DATA message: string
  PUBLIC DATA details: []ErrorDetail?
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

### `ErrorDetail`

```pseudocode!
CLASS ErrorDetail
  PUBLIC DATA id: string?
  PUBLIC DATA message: string?
  PUBLIC DATA path: string?
```

## `GetOfferingsResponseBody`

```pseudocode!
CLASS GetOfferingsResponseBody
  PUBLIC DATA data: []Offering
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `GetBalancesResponseBody`

```pseudocode!
CLASS GetBalancesResponseBody
  PUBLIC DATA data: []Balance
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `GetExchangeResponseBody`

```pseudocode!
CLASS GetExchangeResponseBody
  PUBLIC DATA data: []Message
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `GetExchangesResponseBody`

```pseudocode!
CLASS GetExchangeResponseBody
  PUBLIC DATA data: []string
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `CreateExchangeRequestBody`

```pseudocode!
CLASS CreateExchangeRequestBody
  PUBLIC DATA message: Rfq
  PUBLIC DATA replyTo: string?
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `UpdateExchangeRequestBody`

```pseudocode!
CLASS UpdateExchangeRequestBody
  PUBLIC DATA message: WalletUpdateMessage // Order or Cancel
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

## `ReplyToRequestBody`

```pseudocode!
CLASS ReplyToRequestBody
  PUBLIC DATA message: ReplyToMessage // Quote, OrderStatus or Close
  CONSTRUCTOR from_json_string(json: string)
  METHOD to_json_string(): string
```

# HTTP Client

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

## `Exchange`

```pseudocode!
CLASS Exchange
  PUBLIC DATA rfq: Rfq
  PUBLIC DATA quote: Quote
  PUBLIC DATA order: Order
  PUBLIC DATA cancel: Cancel
  PUBLIC DATA order_statuses: []OrderStatus
  PUBLIC DATA close: Close
```

## `get_exchanges()`

```pseudocode!
FUNCTION get_exchanges(pfi_did_uri: string, bearer_did: BearerDid): []string
```
