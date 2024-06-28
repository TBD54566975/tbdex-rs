# RFC-0001 Standard tbDEX API Design (APID) Document v0.1.0 <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
- [Detailed Design](#detailed-design)
  - [Known Limitations](#known-limitations)
- [Drawbacks](#drawbacks)
- [Alternatives](#alternatives)
- [Prior Art](#prior-art)

# Summary

Lay the initial foundations to drive early implementations, with the expectation to subsequently iterate and build upon.

# Motivation

The driving motivations for a standard API design, adopted across many language implementations, is well defined in the [Web5 RFC-0001](https://github.com/TBD54566975/web5-rs/blob/main/docs/rfcs/rcf-0001/README.md#motivation).

# Detailed Design

This RFC defines version 0.1.0 of the APID, but is expected to change hereafter. 

Many of the design decisions were made from prior implementations, namely [the `tbdex-kt` implementation](https://github.com/TBD54566975/tbdex-kt/). Primary deviations from the [pre-existing `tbdex-kt` implementation](https://github.com/TBD54566975/tbdex-kt/) are three-fold.

1. A difference of namespacing. Although namespacing is not currently explicitly supported in the [Custom DSL](https://github.com/TBD54566975/web5-rs/blob/main/docs/CUSTOM_DSL.md), the hierarchy of the APID markdown file has implicit intentions of namespacing.
2. Migration away from a class `TbdexHttpClient` with static methods, wherein the class served no purpose other than an obscure namespace, and towards functions -- for example, instead of `TbdexHttpClient.getOfferings()` it's now `tbdex.httpclient` (namespaces) and a function `getOfferings()`.
3. No polymorphic base classes for `Resource` and `Message`. Polymorphism is primarily relevant for use cases of developer-defined instances, for example a "bring-your-own" cryptography abstraction wherein the SDK cannot support the full enumeration of possible use cases, but that's not the case for tbDEX resources and messages, both of which are well-defined. For situations of DRY, permissions are encouraged.

## Known Limitations

- Lack of support for `FUNCTIONS` in the [Custom DSL](https://github.com/TBD54566975/web5-rs/blob/main/docs/CUSTOM_DSL.md).
- Lack of support for a JSON Object type in the [Custom DSL](https://github.com/TBD54566975/web5-rs/blob/main/docs/CUSTOM_DSL.md).
- Snake casing and camel casing is inconsistent.
- No "HTTP Server" functionality.

# Drawbacks

None.

# Alternatives

None.

# Prior Art

Heavily inspired by [Web5 RFC-0001](https://github.com/TBD54566975/web5-rs/blob/main/docs/rfcs/rcf-0001/README.md).