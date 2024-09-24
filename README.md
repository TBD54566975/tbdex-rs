[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs.svg?type=shield&issueType=license)](https://app.fossa.com/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs?ref=badge_shield&issueType=license)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs.svg?type=shield&issueType=security)](https://app.fossa.com/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs?ref=badge_shield&issueType=security)

# tbDEX SDK Mono Repo

This monorepo houses the core components of the tbDEX platform containing the core Rust code with Kotlin bindings. tbDEX is a protocol for discovering liquidity and exchanging assets such as fiat money, real world goods, stablecoins or bitcoin. The tbDEX protocol utilizes Decentralized Identitifers (DIDs) and Verifiable Credentials (VCs) to establish the provenance of identity in the real world. The protocol has no opinion on anonymity as a feature or consequence of transactions. Instead, it allows willing counterparties to negotiate and establish the minimum information acceptable for an exchange.


## Table of Contents
- [Features](#features)
- [Usage](#usage)
- [Getting Started](#getting-started)
   - [Cloning](#cloning)
- [Development Prerequisites](#development-prerequisites)
   - [Hermit](#hermit)
- [Building and Testing](#building-and-testing)
- [Binding Process](#binding-process)
- [API Documentation](#api-documentation)
- [Basic Usage](#basic-usage)
   - [DidJwk Creation](#didjwk-creation)
   - [Verifiable Credential Creation & Signing](#verifiable-credential-creation--signing)
   - [Verifiable Presentation Creation & Signing](#verifiable-presentation-creation--signing)
   - [Presentation Exchange](#presentation-exchange)
- [Rust Examples](#rust-examples)
   - [Instantiate a new did:jwk](#instantiate-a-new-didjwk)
   - [Simple Verifiable Credential Creation & Signing](#simple-verifiable-credential-creation--signing)
- [Kotlin Examples](#kotlin-examples)
   - [Instantiate a new did:jwk](#instantiate-a-new-didjwk-1)
   - [Simple Verifiable Credential Creation & Signing](#simple-verifiable-credential-creation--signing-1)


## Features
Comprehensive tbDEX Message Support: Includes all tbDEX resources and message types:
- Resources:
    - Offering: A resource created by a Participating Financial Institution (PFI) to define the terms and requirements for exchanging a specific currency pair. It includes details like exchange rates, payment methods, and any required credentials or claims.

    - Balance: A protected resource that communicates the amounts of each currency held by the PFI on behalf of a customer. It shows the available balance that the customer can transact with.
- Messages:
    - RFQ (Request For Quote): A message sent by a participant (e.g., Alice) to a PFI requesting a quote based on an offering. It specifies the desired amount, selected payment methods, and provides any necessary claims or credentials.

    - Quote: A message sent by a PFI in response to an RFQ. It details the terms of the exchange, including the exchange rate, fees, amounts to be paid and received, and the expiration time of the quote.

    - Order: A message sent by a participant to a PFI indicating acceptance of the quote and intent to proceed with the exchange under the specified terms.

    - OrderInstructions: A message sent by a PFI to a participant providing detailed instructions on how to complete the transaction. This may include payment details, links, or other necessary steps for both payin and payout processes.

    - OrderStatus: A message sent by a PFI to update the participant on the current status of the order. It tracks the progress of the transaction through various stages like pending, initiated, settled, or failed.

    - Close: A message indicating the closure of an exchange. Sent by a PFI, it signifies that the exchange has either been successfully completed or terminated. It may include reasons for the closure and a success flag.

    - Cancel: A message sent by a participant to a PFI to withdraw from an exchange. It expresses the participant's desire to cancel the transaction and may include the reason for cancellation.


## Usage
tbDEX is available
[from Maven Central](https://central.sonatype.com/artifact/xyz.block/tbdex). Instructions for
adding the dependency in a variety of build tools including Maven and Gradle are linked there.

> [!IMPORTANT]
> tbDEX contains transitive dependencies not
> found in Maven Central. To resolve these, add the
> [TBD thirdparty repository](https://blockxyz.jfrog.io/artifactory/tbd-oss-thirdparty-maven2/)
> to your Maven or Gradle config.
>
> For instance, in your Maven `pom.xml`:
>
> ```shell
> <repositories>
>   <repository>
>     <id>tbd-oss-thirdparty</id>
>     <name>tbd-oss-thirdparty</name>
>     <releases>
>       <enabled>true</enabled>
>     </releases>
>     <snapshots>
>       <enabled>false</enabled>
>     </snapshots>
>     <url>https://blockxyz.jfrog.io/artifactory/tbd-oss-thirdparty-maven2/</url>
>   </repository>
> </repositories>
> ```
>
> ...or in your `gradle.settings.kts`:
>
> ```shell
> dependencyResolutionManagement {
>   repositories {
>       mavenCentral()
>       // Thirdparty dependencies of TBD projects not in Maven Central
>       maven("https://blockxyz.jfrog.io/artifactory/tbd-oss-thirdparty-maven2/")
> }
> ```

## Getting Started

To start developing applications and services with the tbDEX RS SDK, the following steps will guide
you through setting up your local development environment.

tbDEX is available
[from Maven Central](https://central.sonatype.com/artifact/xyz.block/tbdex). Instructions for
adding the dependency in a variety of build tools including Maven and Gradle are linked there.

For detailed documentation on usage refer to the
[API reference documentation](docs/API_DESIGN.md). Additionally, comprehensive
guides can be found at the [TBD Developer site](https://developer.tbd.website/docs/) to enhance
your understanding of the underlying concepts and how to implement them effectively.

To dive deeper into the actual tbDEX protocol refer to the
[Tbdex protocol documentation](https://github.com/TBD54566975/tbdex/blob/main/specs/protocol/README.md).

### Cloning

This repository uses git submodules. To clone this repo with submodules:
```sh
git clone --recurse-submodules git@github.com:TBD54566975/tbdex-rs.git
```

Or to add submodules after cloning:
```sh
git submodule update --init
```

## Development Prerequisites

### Hermit

This project uses hermit to manage tooling like the Rust compiler, Java Development Kit and Maven project management system.
See [this page](https://cashapp.github.io/hermit/usage/get-started/) to set up Hermit on your machine - make sure to
download the open source build and activate it for the project.

Once you've installed Hermit and before running builds on this repo,
run from the root:

```shell
source ./bin/activate-hermit
```

This will set your environment up correctly in the
terminal emulator you're on. Executing `just` commands should "just work", no
matter the underlying tooling used (ie. `rustc`, `cargo`, `mvn`, `java`, etc).

## Building and Testing

To run, find a build target from the table below and use `just`:

```shell
$> just [recipe]
```

| Command       | Description |
| ------------- | ----------- |
| `setup`       | Initalizes the environment, including `git` submodules, `rustup`, etc.  |
| `build`       | Builds the Rust core |
| `test`        | Tests the Rust core |
| `lint`        | Performs code formatting on the Rust core |
| `bind`        | Builds all language bindings |
| `bind-kotlin` | Builds the Kotlin language bindings |
| `test-bound` | Tests all language bindings |
| `test-kotlin` | Tests the Kotlin language bindings |

For instance:

```shell
$> just build
```

## Binding Process

The binding process follows these key steps:

1. **Core Rust Development**
   All the core logic for working with the base tbDEX resources and messages, and cryptographic signing and verification is implemented in Rust. Rust is chosen as the core layer for its memory safety, performance, and cross-platform capabilities.

2. **Building the Kotlin Bindings**  
   The Kotlin bindings are generated from the core Rust code and live in the `bound/kt` directory. These bindings allow Kotlin applications to access the functionality of the core Rust libraries through idiomatic Kotlin APIs.

3. **Packaging & Distribution**  
   The Kotlin bindings are packaged and distributed as a Kotlin library, which can be imported and used in Kotlin applications just like any other dependency.

## API Documentation
For the full detailed API design and usage examples, refer to the [API Design Document](docs/API_DESIGN.md)

## Basic Usage

The SDK allows developers to work with tbDEX resources and messages along with cryptographic signing and verification. Below are the key use cases:

## Rust Examples
### Create and sign new RFQ

```rust
/// Create the Bearer DID (you might need a proper function to generate the DID)
let bearer_did = DidJwk::create(None).unwrap();

/// Create the RFQ message
let mut rfq = Rfq::create(
    "did:test:pfi",
    &bearer_did.did.uri,
    &CreateRfqData {
        offering_id: "offering_123".to_string(),
        payin: CreateSelectedPayinMethod {
            kind: "BTC".to_string(),
            payment_details: Some(serde_json::json!({"tmp": "payment-details"})),
            amount: "101".to_string(),
        },
        payout: CreateSelectedPayoutMethod {
            kind: "BTC".to_string(),
            payment_details: Some(serde_json::json!({"tmp": "payment-details"})),
        },
        claims: vec!["some-claim".to_string()],
    },
    None,
    None,
)
.unwrap();

// Sign the RFQ with the Bearer DID
rfq.sign(&bearer_did).unwrap();
```

### Decode and verify RFQ

```rust
        let rfq_json_string = rfq.to_json_string().unwrap();
        let parsed_rfq: Rfq = Rfq::from_json_string(&rfq_json_string).unwrap();

        parsed_rfq.verify_present_private_data().unwrap();
```

## Kotlin Examples
### Create and sign new RFQ

```kotlin
// Create the Bearer DID (you might need a proper function to generate the DID)
val bearerDid = BearerDid.create() // Assuming BearerDid has a create() method similar to Rust

// Create the RFQ message
val rfq = Rfq.create(
    to = "did:test:pfi",
    from = bearerDid.uri,
    createRfqData = CreateRfqData(
        offeringId = "offering_123",
        payin = CreateSelectedPayinMethod(
            kind = "BTC",
            paymentDetails = mapOf("tmp" to "payment-details"),
            amount = "101"
        ),
        payout = CreateSelectedPayoutMethod(
            kind = "BTC",
            paymentDetails = mapOf("tmp" to "payment-details")
        ),
        claims = listOf("some-claim")
    )
)

// Sign the RFQ with the Bearer DID
rfq.sign(bearerDid)
```

### Decode and verify RFQ
```kotlin
val rfqJsonString: String = rfq.toJsonString()
val parsedRfq: Rfq = Rfq.fromJsonString(rfqJsonString)

parsedRfq.verifyPresentPrivateData()
```


## Dependencies
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B588%2Fgithub.com%2FTBD54566975%2Ftbdex-rs?ref=badge_large&issueType=license)
