# Kotlin Client w/ `replyTo` Example

The intention of this project is to showcase a simple end-to-end example of a tbDEX exchange, from the perspective of the *Client* (often referred to as "*Alice*" elsewhere), with the utilization of the `replyTo` field during the [Create Exchange](https://github.com/TBD54566975/tbdex/tree/main/specs/http-api#create-exchange), or said differently, this example consists of an HTTP server for which the PFI is expected to callback to for the Quote, Order Status, and Close messages.

## Tutorial

### Configure 

---

This project comes fully functional with fake data out-of-the-box. However, as a developer, you're able to add & edit ...

---

TODO 
- automate the `mvn install` for tbdex-core-kt


- values are already configured and checked-into source control
- use the CLI to regenerate values, and set env vars
- "effectively, the server is a proxy in it's current form, but for enterprise use cases it is expected to be an abstraction layer in between the user experience and the PFI"

config values:
- PFI DID
- VC
- portable did