# tbDEX API Design (APID) <!-- omit in toc -->

**Last Updated:** June 13, 2024

**Custom DSL Version:** 0.1.0

- [Resources](#resources)
  - [`Resource`](#resource)
  - [`ResourceMetadata`](#resourcemetadata)
  - [Offering](#offering)
    - [`OfferingData`](#offeringdata)
- [Messages](#messages)

# Resources

## `Resource`

```pseudocode!
INTERFACE Resource
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

## `ResourceMetadata`

```pseudocode!
CLASS ResourceMetadata
  PUBLIC DATA kind: string
  /// 🚧 more members
```

## Offering

```pseudocode!
CLASS Offering IMPLEMENTS Resource
  CONSTRUCTOR(from: string, data: OfferingData, protocol: string)
  CONSTRUCTOR(json: string)
  METHOD sign(bearer_did: BearerDid)
  METHOD verify(): bool
```

### `OfferingData`

```pseudocode!
CLASS OfferingData
  PUBLIC DATA description: string
  /// 🚧 more members
```

# Messages

...