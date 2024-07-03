# hosted-wallet-kt

The intention of this project is to showcase a simple end-to-end example of a tbDEX exchange, primarily from the perspective of the *Client* (often referred to as "*Alice*" elsewhere), with the utilization of the `replyTo` field during the [Create Exchange](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#create-exchange), or said differently, this example consists of an HTTP server for which the PFI is expected to callback to for the Quote, Order Status, and Close messages.

# How-To

## 1. Install `tbdex-core-kt` to local maven repository

> [!NOTE]
>
> This is only necessary initially, or upon any changes made to the `tbdex-core-kt` project.

```shell
(cd ../../bound/kt; mvn install)
```

## 2. Run `pfi/` in one terminal

> [!NOTE]
>
> Wait for the log message, "*PFI server running...*"

```shell
gradle :pfi:run
```

## 3. Run `wallet/` in a second terminal

```shell
gradle :wallet:run
```

You should begin to see log outputs beginning with:

```shell
1. Fetching offerings...
Successfully fetched offering_01j1v9pk2zeh9bsstkhrhwtgrc

2. Creating exchange...
```

# About

The composition of this project is 2-part:

1. `pfi/`
2. `wallet/`

The `pfi/` project is a simple HTTP server which defines & hosts a single fake offering (`GET /offerings`), exposes endpoints for exchange creation (`POST /exchanges`), AKA an "RFQ", and an endpoint for Order sumbission (`PUT /exchanges/:id`). The `pfi/` server does not utilize a database, so exchanges only persist for the runtime of the server. The `pfi/` server does not expose the tbDEX HTTP endpoints for [Submit Close](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#submit-close) (the `PUT /exchanges/:id` endpoint solely processes Order messages), [Get Exchange](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#get-exchange) (`GET /exchanges/:id`), [List Exchanges](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#list-exchanges) (`GET /exchanges`), nor [List Balances](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#list-balances) (`GET /balances`). In other words, the `/pfi` project hosts only the necessary endpoints for a "happy-path" integration with the `/wallet` project.

The `pfi/` project ***does*** support (and indeed requires) [the `replyTo` callback](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#callbacks) which is first sent in the RFQ.

The `wallet/` project is a simple 7-step `main` function which begins with setting up the Webhook server (for receiving Quote, OrderStatuses, and Close messages from the PFI via the `replyTo`), followed by fetching Offerings, and then stepping through the tbDEX exchange one message at a time all the way to receiving the Close with a `success: true` to successfully complete the exchange. 

Both projects contain hard-coded tbDEX exchange data, for example the `Offering` made available in the `pfi/` project includes payment details which are respectively hard-coded in the `wallet/` project. The "hard-coded" implementation within this example exists for the sake of simplifying comprehension of the concepts whilst also showcasing a comprehensive integration.

Both projects are configured via their `resource/.env` file. It is possible to use [the `web5` CLI tool ](https://github.com/TBD54566975/web5-rs/tree/main/crates/web5_cli) to regenerate these values, however attempting to utilize the `wallet/` with a PFI other than the `pfi/` project, in it's current state, will fail due to the hard-coded tbDEX exchage data. 
