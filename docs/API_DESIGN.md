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
    - [`OrderInstructionsData`](#orderinstructionsdata)
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
  - [`get_exchange_ids()`](#get_exchange_ids)
    - [`GetExchangeIdsQueryParams`](#getexchangeidsqueryparams)

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
/// Represents different kinds of resources in tbDEX.
ENUM ResourceKind
  /// Offering resource type.
  offering,
  /// Balance resource type.
  balance,
```

## `ResourceMetadata`

```pseudocode!
/// Metadata about a resource including its origin and version.
CLASS ResourceMetadata
  /// kind is the type of resource (e.g., offering, balance).
  PUBLIC DATA kind: string
  /// from indicates who created the resource.
  PUBLIC DATA from: string
  /// to indicates the intended recipient of the resource.
  PUBLIC DATA to: string
  /// id is a unique identifier for the resource.
  PUBLIC DATA id: string
  /// protocol specifies the protocol version used.
  PUBLIC DATA protocol: string
  /// createdAt is the timestamp when the resource was created.
  PUBLIC DATA createdAt: string
  /// updatedAt is the optional timestamp for when the resource was last updated.
  PUBLIC DATA updatedAt: string?
```

## `Offering`

```pseudocode!
/// Represents an offering resource in tbDEX.
CLASS Offering IMPLEMENTS Resource
  /// metadata contains information about the offering.
  PUBLIC DATA metadata: ResourceMetadata
  /// data contains the specific details of the offering.
  PUBLIC DATA data: OfferingData
  /// signature is used to verify the authenticity of the offering.
  PUBLIC DATA signature: string
  /// create initializes an Offering with specified metadata and data.
  CONSTRUCTOR create(from: string, data: OfferingData, protocol: string?)
  /// from_json_string constructs an Offering from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// sign creates a digital signature for the offering using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// verify checks the validity of the offering's signature.
  METHOD verify(): Error?
  /// to_json_string converts the Offering to a JSON string.
  METHOD to_json_string(): string
```

### `OfferingData`

```pseudocode!
/// Contains the data specific to an offering, including payin and payout details.
CLASS OfferingData
  /// description provides details about what is being offered.
  PUBLIC DATA description: string
  /// payoutUnitsPerPayinUnit defines the exchange rate between payin and payout currencies.
  PUBLIC DATA payoutUnitsPerPayinUnit: string
  /// payin contains details about the payin currency and methods.
  PUBLIC DATA payin: PayinDetails
  /// payout contains details about the payout currency and methods.
  PUBLIC DATA payout: PayoutDetails
  /// requiredClaims specifies any claims required for the offering.
  PUBLIC DATA requiredClaims: PresentationDefinition?
  /// cancellation contains the cancellation policy for the offering.
  PUBLIC DATA cancellation: CancellationDetails
```

### `PayinDetails`

```pseudocode!
/// Details about the payin currency and associated methods.
CLASS PayinDetails
  /// currencyCode is the ISO code for the payin currency.
  PUBLIC DATA currencyCode: string
  /// min is the optional minimum amount required for the payin.
  PUBLIC DATA min: string?
  /// max is the optional maximum amount allowed for the payin.
  PUBLIC DATA max: string?
  /// methods lists the available payin methods.
  PUBLIC DATA methods: []PayinMethod
```

### `PayinMethod`

```pseudocode!
/// Describes a method for making payments to the PFI.
CLASS PayinMethod
  /// kind is a unique identifier for the payin method (e.g., DEBIT_CARD).
  PUBLIC DATA kind: string
  /// name is an optional name for the payin method.
  PUBLIC DATA name: string?
  /// description provides additional information about the payin method.
  PUBLIC DATA description: string?
  /// group categorizes the payin method (e.g., Mobile Money).
  PUBLIC DATA group: string?
  /// requiredPaymentDetails is a JSON schema specifying required fields for the payment method.
  PUBLIC DATA requiredPaymentDetails: JsonNode? /// 🚧
  /// fee indicates any fee associated with the payin method.
  PUBLIC DATA fee: string?
  /// min specifies the minimum amount required to use this payment method.
  PUBLIC DATA min: string?
  /// max specifies the maximum amount allowed with this payment method.
  PUBLIC DATA max: string?
```

### `PayoutDetails`

```pseudocode!
/// Details about the payout currency and associated methods.
CLASS PayoutDetails
  /// currencyCode is the ISO code for the payout currency.
  PUBLIC DATA currencyCode: string
  /// min is the optional minimum amount allowed for the payout.
  PUBLIC DATA min: string?
  /// max is the optional maximum amount allowed for the payout.
  PUBLIC DATA max: string?
  /// methods lists the available payout methods.
  PUBLIC DATA methods: []PayoutMethod
```

### `PayoutMethod`

```pseudocode!
/// Describes a method for receiving payments from the PFI.
CLASS PayoutMethod
  /// kind is a unique identifier for the payout method (e.g., BTC_ADDRESS).
  PUBLIC DATA kind: string
  /// name is an optional name for the payout method.
  PUBLIC DATA name: string?
  /// description provides additional information about the payout method.
  PUBLIC DATA description: string?
  /// group categorizes the payout method (e.g., Mobile Money).
  PUBLIC DATA group: string?
  /// requiredPaymentDetails is a JSON schema specifying required fields for the payout method.
  PUBLIC DATA requiredPaymentDetails: JsonNode? /// 🚧
  /// fee indicates any fee associated with the payout method.
  PUBLIC DATA fee: string?
  /// min specifies the minimum amount required for this payout method.
  PUBLIC DATA min: string?
  /// max specifies the maximum amount allowed with this payout method.
  PUBLIC DATA max: string?
  /// estimatedSettlementTime is the estimated time for settling an order, in seconds.
  PUBLIC DATA estimatedSettlementTime: int
```

### `CancellationDetails`

```pseudocode!
/// Contains details about the cancellation policy for an offering.
CLASS CancellationDetails
  /// enabled indicates if cancellation is allowed for this offering.
  PUBLIC DATA enabled: bool
  /// termsUrl is a link to a page with cancellation terms.
  PUBLIC DATA termsUrl: string?
  /// terms is a human-readable description of the cancellation terms.
  PUBLIC DATA terms: string?
```

## `Balance`

```pseudocode!
/// Represents a balance resource showing the available currency for a customer.
CLASS Balance IMPLEMENTS Resource
  /// metadata contains information about the balance resource.
  PUBLIC DATA metadata: ResourceMetadata
  /// data contains the specific details of the balance.
  PUBLIC DATA data: BalanceData
  /// signature is used to verify the authenticity of the balance.
  PUBLIC DATA signature: string
  /// create initializes a Balance with specified metadata and data.
  CONSTRUCTOR create(from: string, data: BalanceData, protocol: string?)
  /// from_json_string constructs a Balance from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// sign creates a digital signature for the balance using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// verify checks the validity of the balance's signature.
  METHOD verify(): Error?
  /// to_json_string converts the Balance to a JSON string.
  METHOD to_json_string(): string
```

### `BalanceData`

```pseudocode!
/// Contains the data specific to a balance, including currency and available amount.
CLASS BalanceData
  /// currencyCode is the ISO code for the currency of the balance.
  PUBLIC DATA currencyCode: string
  /// available is the amount of currency available for transactions.
  PUBLIC DATA available: string
```

# Messages

## `MessageKind`

```pseudocode!
/// Represents different kinds of messages in tbDEX.
ENUM MessageKind
  /// RFQ message kind.
  rfq,
  /// Quote message kind.
  quote,
  /// Order message kind.
  order,
  /// OrderInstructions message kind.
  orderinstructions,
  /// Cancel message kind.
  cancel,
  /// OrderStatus message kind.
  orderstatus,
  /// Close message kind.
  close,
```

## `MessageMetadata`

```pseudocode!
/// Metadata about a message, including its sender, recipient, and type.
CLASS MessageMetadata
  /// from is the DID of the message sender.
  PUBLIC DATA from: string
  /// to is the DID of the message recipient.
  PUBLIC DATA to: string
  /// kind specifies the type of the message (e.g., rfq, quote).
  PUBLIC DATA kind: MessageKind
  /// id is a unique identifier for the message.
  PUBLIC DATA id: string
  /// exchangeId is the ID for the exchange between parties.
  PUBLIC DATA exchangeId: string
  /// externalId is an optional arbitrary ID for the caller's reference.
  PUBLIC DATA externalId: string?
  /// createdAt is the timestamp when the message was created.
  PUBLIC DATA createdAt: string
  /// protocol specifies the protocol version used.
  PUBLIC DATA protocol: string
```

## `Message`

```pseudocode!
/// Represents a general message interface in tbDEX.
INTERFACE Message
```

## `WalletUpdateMessage`

```pseudocode!
/// Represents a wallet update message interface.
INTERFACE WalletUpdateMessage
```

## `ReplyToMessage`

```pseudocode!
/// Represents a reply to a message interface.
INTERFACE ReplyToMessage
```

## `Rfq`

```pseudocode!
/// Represents a Request For Quote (RFQ) message.
CLASS Rfq IMPLEMENTS Message
  /// metadata contains information about the RFQ.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the RFQ.
  PUBLIC DATA data: RfqData
  /// privateData contains sensitive information related to the RFQ.
  PUBLIC DATA privateData: RfqPrivateData
  /// signature is used to verify the authenticity of the RFQ.
  PUBLIC DATA signature: string
  /// create initializes an RFQ with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, rfqData: CreateRfqData, protocol: string?, externalId: string?)
  /// from_json_string constructs an RFQ from a JSON string.
  CONSTRUCTOR from_json_string(json: string, requireAllPrivateData: bool?)
  /// verify checks the validity of the RFQ's signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the RFQ using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the RFQ to a JSON string.
  METHOD to_json_string(): string
```

### `CreateRfqData`

```pseudocode!
/// Contains the data needed to create an RFQ.
CLASS CreateRfqData
  /// offeringId is the ID of the offering being quoted.
  PUBLIC DATA offeringId: string
  /// payin details for the RFQ.
  PUBLIC DATA payin: CreateSelectedPayinMethod
  /// payout details for the RFQ.
  PUBLIC DATA payout: CreateSelectedPayoutMethod
  /// claims is a list of claims relevant to the RFQ.
  PUBLIC DATA claims: []string
```

### `CreateSelectedPayinMethod`

```pseudocode!
/// Contains details about the selected payin method for an RFQ.
CLASS CreateSelectedPayinMethod
  /// kind specifies the type of payin method (e.g., DEBIT_CARD).
  PUBLIC DATA kind: string
  /// paymentDetails is a map of details required for the payin method.
  PUBLIC DATA paymentDetails: Map<string, JsonNode>?
  /// amount specifies the amount of currency to be paid in.
  PUBLIC DATA amount: string
```

### `CreateSelectedPayoutMethod`

```pseudocode!
/// Contains details about the selected payout method for an RFQ.
CLASS CreateSelectedPayoutMethod
  /// kind specifies the type of payout method (e.g., BTC_ADDRESS).
  PUBLIC DATA kind: string
  /// paymentDetails is a map of details required for the payout method.
  PUBLIC DATA paymentDetails: Map<string, JsonNode>
```

### `RfqData`

```pseudocode!
/// Contains the data specific to an RFQ.
CLASS RfqData
  /// offeringId is the ID of the offering being quoted.
  PUBLIC DATA offeringId: string
  /// payin details for the RFQ.
  PUBLIC DATA payin: SelectedPayinMethod
  /// payout details for the RFQ.
  PUBLIC DATA payout: SelectedPayoutMethod
  /// claimsHash is a hash of the claims presented in the RFQ.
  PUBLIC DATA claimsHash: string?
```

### `SelectedPayinMethod`

```pseudocode!
/// Contains details about the selected payin method.
CLASS SelectedPayinMethod
  /// kind specifies the type of payin method (e.g., DEBIT_CARD).
  PUBLIC DATA kind: string
  /// paymentDetailsHash is a hash of the payment details for the payin method.
  PUBLIC DATA paymentDetailsHash: string?
  /// amount specifies the amount of currency to be paid in.
  PUBLIC DATA amount: string
```

### `SelectedPayoutMethod`

```pseudocode!
/// Contains details about the selected payout method.
CLASS SelectedPayoutMethod
  /// kind specifies the type of payout method (e.g., BTC_ADDRESS).
  PUBLIC DATA kind: string
  /// paymentDetailsHash is a hash of the payment details for the payout method.
  PUBLIC DATA paymentDetailsHash: string?
```

### `RfqPrivateData`

```pseudocode!
/// Contains private data related to an RFQ.
CLASS RfqPrivateData
  /// salt is a randomly generated value used in hashing.
  PUBLIC DATA salt: string
  /// payin contains private details for the payin method.
  PUBLIC DATA payin: PrivatePaymentDetails?
  /// payout contains private details for the payout method.
  PUBLIC DATA payout: PrivatePaymentDetails?
  /// claims contains private claims relevant to the RFQ.
  PUBLIC DATA claims: []string?
```

### `PrivatePaymentDetails`

```pseudocode!
/// Contains private payment details for an RFQ.
CLASS PrivatePaymentDetails
  /// paymentDetails is a map of the actual payment details.
  PUBLIC DATA paymentDetails: Map<string, JsonNode>?
```

## `Quote`

```pseudocode!
/// Represents a Quote message in response to an RFQ.
CLASS Quote IMPLEMENTS Message, ReplyToMessage
  /// metadata contains information about the Quote.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the Quote.
  PUBLIC DATA data: QuoteData
  /// signature is used to verify the authenticity of the Quote.
  PUBLIC DATA signature: string
  /// create initializes a Quote with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, quoteData: QuoteData, protocol: string?, externalId: string?)
  /// from_json_string constructs a Quote from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the Quote's signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the Quote using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the Quote to a JSON string.
  METHOD to_json_string(): string
```

### `QuoteData`

```pseudocode!
/// Contains the data specific to a Quote.
CLASS QuoteData
  /// expiresAt is the timestamp when the Quote expires.
  PUBLIC DATA expiresAt: string
  /// payin contains details about the payin currency and methods.
  PUBLIC DATA payin: QuoteDetails
  /// payout contains details about the payout currency and methods.
  PUBLIC DATA payout: QuoteDetails
```

### `QuoteDetails`

```pseudocode!
/// Contains details about a Quote, including costs and fees.
CLASS QuoteDetails
  /// currencyCode is the ISO code for the currency.
  PUBLIC DATA currencyCode: string
  /// subtotal is the amount before fees.
  PUBLIC DATA subtotal: string
  /// total is the total amount including fees.
  PUBLIC DATA total: string
  /// fee is the optional fee charged for the Quote.
  PUBLIC DATA fee: string?
```

## `Order`

```pseudocode!
/// Represents an Order message in response to a Quote.
CLASS Order IMPLEMENTS Message, WalletUpdateMessage
  /// metadata contains information about the Order.
  PUBLIC DATA metadata: MessageMetadata
  /// signature is used to verify the authenticity of the Order.
  PUBLIC DATA signature: string
  /// create initializes an Order with specified metadata and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, protocol: string?, externalId: string?)
  /// from_json_string constructs an Order from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the Order's signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the Order using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the Order to a JSON string.
  METHOD to_json_string(): string
```

## `OrderInstructions`

```pseudocode!
/// Represents instructions for completing an Order.
CLASS OrderInstructions IMPLEMENTS Message, ReplyToMessage
  /// metadata contains information about the OrderInstructions.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the OrderInstructions.
  PUBLIC DATA data: OrderInstructionsData
  /// signature is used to verify the authenticity of the OrderInstructions.
  PUBLIC DATA signature: string
  /// create initializes OrderInstructions with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, orderInstructionsData: OrderInstructionsData, protocol: string?, externalId: string?)
  /// from_json_string constructs OrderInstructions from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the OrderInstructions' signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the OrderInstructions using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the OrderInstructions to a JSON string.
  METHOD to_json_string(): string
```

### `OrderInstructionsData`

```pseudocode!
/// Contains the data specific to OrderInstructions, including payment and payout instructions.
CLASS OrderInstructionsData
  /// payin contains payment instructions for how to pay the PFI.
  PUBLIC DATA payin: PaymentInstruction
  /// payout contains payment instructions for how the PFI will pay Alice.
  PUBLIC DATA payout: PaymentInstruction
```

### `PaymentInstruction`

```pseudocode!
/// Contains instructions for making a payment or receiving payment.
CLASS PaymentInstruction
  /// link is an optional URL for making a payment or receiving payment.
  PUBLIC DATA link: string?
  /// instruction provides additional details on how to make or receive payment.
  PUBLIC DATA instruction: string?
```

## `Cancel`

```pseudocode!
/// Represents a Cancel message used to back out of an exchange.
CLASS Cancel IMPLEMENTS Message, WalletUpdateMessage
  /// metadata contains information about the Cancel.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the Cancel.
  PUBLIC DATA data: CancelData
  /// signature is used to verify the authenticity of the Cancel.
  PUBLIC DATA signature: string
  /// create initializes a Cancel with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, cancelData: CancelData, protocol: string?, externalId: string?)
  /// from_json_string constructs a Cancel from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the Cancel's signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the Cancel using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the Cancel to a JSON string.
  METHOD to_json_string(): string
```

### `CancelData`

```pseudocode!
/// Contains the data specific to a Cancel message.
CLASS CancelData
  /// reason provides the reason for canceling the exchange.
  PUBLIC DATA reason: string
```

## `OrderStatus`

```pseudocode!
/// Represents the status of an Order.
CLASS OrderStatus IMPLEMENTS Message, ReplyToMessage
  /// metadata contains information about the OrderStatus.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the OrderStatus.
  PUBLIC DATA data: OrderStatusData
  /// signature is used to verify the authenticity of the OrderStatus.
  PUBLIC DATA signature: string
  /// create initializes an OrderStatus with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, orderStatusData: OrderStatusData, protocol: string?, externalId: string?)
  /// from_json_string constructs an OrderStatus from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the OrderStatus' signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the OrderStatus using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the OrderStatus to a JSON string.
  METHOD to_json_string(): string
```

### `OrderStatusData`

```pseudocode!
/// Contains the data specific to an OrderStatus.
CLASS OrderStatusData
  /// orderStatus indicates the current status of the Order.
  PUBLIC DATA orderStatus: string
```

## `Close`

```pseudocode!
/// Represents a Close message indicating the end of an exchange.
CLASS Close IMPLEMENTS Message, ReplyToMessage
  /// metadata contains information about the Close.
  PUBLIC DATA metadata: MessageMetadata
  /// data contains the specific details of the Close.
  PUBLIC DATA data: CloseData
  /// signature is used to verify the authenticity of the Close.
  PUBLIC DATA signature: string
  /// create initializes a Close with specified metadata, data, and optional parameters.
  CONSTRUCTOR create(to: string, from: string, exchangeId: string, closeData: CloseData, protocol: string?, externalId: string?)
  /// from_json_string constructs a Close from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// verify checks the validity of the Close's signature.
  METHOD verify(): Error?
  /// sign creates a digital signature for the Close using the given BearerDid.
  METHOD sign(bearer_did: BearerDid): Error?
  /// to_json_string converts the Close to a JSON string.
  METHOD to_json_string(): string
```

### `CloseData`

```pseudocode!
/// Contains the data specific to a Close message.
CLASS CloseData
  /// reason provides an optional reason for closing the exchange.
  PUBLIC DATA reason: string?
  /// success indicates whether the exchange was successfully completed.
  PUBLIC DATA success: bool?
```

# HTTP

## `ErrorResponseBody`

[Reference.](https:///github.com/TBD54566975/tbdex/tree/main/specs/http-api#error-responses)

```pseudocode!
/// Represents the error response body for HTTP errors.
CLASS ErrorResponseBody
  /// message provides a description of the error.
  PUBLIC DATA message: string
  /// details contains additional information about the error.
  PUBLIC DATA details: []ErrorDetail?
  /// from_json_string constructs an ErrorResponseBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the ErrorResponseBody to a JSON string.
  METHOD to_json_string(): string
```

### `ErrorDetail`

```pseudocode!
/// Contains details about an error within the error response body.
CLASS ErrorDetail
  /// id is an optional identifier for the error detail.
  PUBLIC DATA id: string?
  /// message provides an optional message about the error detail.
  PUBLIC DATA message: string?
  /// path indicates the optional path related to the error.
  PUBLIC DATA path: string?
```

## `GetOfferingsResponseBody`

```pseudocode!
/// Represents the response body for retrieving offerings.
CLASS GetOfferingsResponseBody
  /// data contains a list of offerings.
  PUBLIC DATA data: []Offering
  /// from_json_string constructs a GetOfferingsResponseBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the GetOfferingsResponseBody to a JSON string.
  METHOD to_json_string(): string
```

## `GetBalancesResponseBody`

```pseudocode!
/// Represents the response body for retrieving balances.
CLASS GetBalancesResponseBody
  /// data contains a list of balances.
  PUBLIC DATA data: []Balance
  /// from_json_string constructs a GetBalancesResponseBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the GetBalancesResponseBody to a JSON string.
  METHOD to_json_string(): string
```

## `GetExchangeResponseBody`

```pseudocode!
/// Represents the response body for retrieving a specific exchange.
CLASS GetExchangeResponseBody
  /// data contains the exchange messages.
  PUBLIC DATA data: []Message
  /// from_json_string constructs a GetExchangeResponseBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the GetExchangeResponseBody to a JSON string.
  METHOD to_json_string(): string
```

## `GetExchangesResponseBody`

```pseudocode!
/// Represents the response body for retrieving a specific exchange.
CLASS GetExchangeResponseBody
  /// data contains the exchange messages.
  PUBLIC DATA data: []Message
  /// from_json_string constructs a GetExchangeResponseBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the GetExchangeResponseBody to a JSON string.
  METHOD to_json_string(): string
```

## `CreateExchangeRequestBody`

```pseudocode!
/// Represents the request body for creating an exchange.
CLASS CreateExchangeRequestBody
  /// message is the RFQ message to initiate the exchange.
  PUBLIC DATA message: Rfq
  /// replyTo is an optional ID to which this request is a response.
  PUBLIC DATA replyTo: string?
  /// from_json_string constructs a CreateExchangeRequestBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the CreateExchangeRequestBody to a JSON string.
  METHOD to_json_string(): string
```

## `UpdateExchangeRequestBody`

```pseudocode!
/// Represents the request body for updating an exchange.
CLASS UpdateExchangeRequestBody
  /// message is the wallet update message (Order or Cancel).
  PUBLIC DATA message: WalletUpdateMessage /// Order or Cancel
  /// from_json_string constructs an UpdateExchangeRequestBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the UpdateExchangeRequestBody to a JSON string.
  METHOD to_json_string(): string
```

## `ReplyToRequestBody`

```pseudocode!
/// Represents the request body for replying to a message.
CLASS ReplyToRequestBody
  /// message is the reply message (Quote, OrderStatus, or Close).
  PUBLIC DATA message: ReplyToMessage /// Quote, OrderStatus or Close
  /// from_json_string constructs a ReplyToRequestBody from a JSON string.
  CONSTRUCTOR from_json_string(json: string)
  /// to_json_string converts the ReplyToRequestBody to a JSON string.
  METHOD to_json_string(): string
```

# HTTP Client

## `get_offerings()`

```pseudocode!
/// Retrieves a list of offerings from the specified PFI DID URI.
FUNCTION get_offerings(pfi_did_uri: string): []Offering
```

## `get_balances()`

```pseudocode!
/// Retrieves the balances for a specified PFI DID URI and bearer DID.
FUNCTION get_balances(pfi_did_uri: string, bearer_did: BearerDid): []Balance
```

## `create_exchange()`

```pseudocode!
/// Creates a new exchange with the specified RFQ and optional reply ID.
FUNCTION create_exchange(rfq: Rfq, reply_to: string?)
```

## `submit_order()`

```pseudocode!
/// Submits an order to the specified exchange.
FUNCTION submit_order(order: Order)
```

## `submit_close()`

```pseudocode!
/// Submits a close message to end the exchange.
FUNCTION submit_close(close: Close)
```

## `get_exchange()`

```pseudocode!
/// Retrieves details of a specific exchange using the PFI DID URI, bearer DID, and exchange ID.
FUNCTION get_exchange(pfi_did_uri: string, bearer_did: BearerDid, exchange_id: string): Exchange
```

## `Exchange`

```pseudocode!
/// Represents an exchange containing various message types.
CLASS Exchange
  /// rfq contains the Request For Quote message.
  PUBLIC DATA rfq: Rfq
  /// quote contains the Quote message.
  PUBLIC DATA quote: Quote
  /// order contains the Order message.
  PUBLIC DATA order: Order
  /// cancel contains the Cancel message.
  PUBLIC DATA cancel: Cancel
  /// order_statuses contains the list of OrderStatus messages.
  PUBLIC DATA order_statuses: []OrderStatus
  /// close contains the Close message.
  PUBLIC DATA close: Close
```

## `get_exchange_ids()`

```pseudocode!
/// Retrieves a list of exchange IDs based on the specified query parameters.
FUNCTION get_exchange_ids(
  pfi_did_uri: string, 
  bearer_did: BearerDid, 
  query_params: GetExchangeIdsQueryParams?): []string
```

### `GetExchangeIdsQueryParams`

```pseudocode!
/// Contains query parameters for retrieving exchange IDs.
CLASS GetExchangeIdsQueryParams
  /// pagination_offset specifies the starting point for pagination.
  PUBLIC DATA pagination_offset: int
  /// pagination_limit specifies the maximum number of results to return.
  PUBLIC DATA pagination_limit: int
```