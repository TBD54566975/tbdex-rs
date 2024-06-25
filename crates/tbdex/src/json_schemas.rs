pub const GIT_COMMIT_HASH: &str = "96a1a7164e8e0a608befa31b6cf0c9a4e5cc0f07";
pub const RESOURCE_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/resource.schema.json",
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "metadata": {
      "type": "object",
      "properties": {
        "from": {
          "$ref": "definitions.json\#/definitions/did",
          "description": "The PFI's DID"
        },
        "kind": {
          "type": "string",
          "enum": ["offering", "balance"],
          "description": "The resource kind (e.g. Offering)"
        },
        "id": {
          "type": "string",
          "description": "The resource id"
        },
        "createdAt": {
          "type": "string",
          "description": "When the resource was created at. Expressed as ISO8601"
        },
        "updatedAt": {
          "type": "string",
          "description": "When the resource was last updated. Expressed as ISO8601"
        },
        "protocol": {
          "type": "string",
          "description": "Version of the protocol in use (x.x format)"
        }
      },
      "required": ["from", "kind", "id", "createdAt", "protocol"],
      "description": "The metadata object contains fields about the resource and is present for every tbdex resources of all types."
    },
    "data": {
      "description": "The actual resource content",
      "type": "object"
    },
    "signature": {
      "type": "string",
      "description": "Signature that verifies that authenticity and integrity of a message"
    }
  },
  "required": ["metadata", "data", "signature"],
  "description": "ResourceModel"
}
"#;
pub const BALANCE_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/balance.schema.json",
  "type": "object",
  "properties": {
    "additionalProperties": false,
    "currencyCode": {
      "type": "string",
      "description": "ISO 4217 currency code string"
    },
    "available": {
      "$ref": "definitions.json\#/definitions/decimalString",
      "description": "The amount available to be transacted with"
    }
  },
  "required": [
    "currencyCode",
    "available"
  ]
}
"#;
pub const OFFERING_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/offering.schema.json",
  "type": "object",
  "properties": {
    "additionalProperties": false,
    "description": {
      "type": "string",
      "description": "Brief description of what is being offered."
    },
    "payin": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "currencyCode": {
          "type": "string",
          "description": "ISO 4217 currency code string"
        },
        "min": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "Minimum amount of currency that can be requested"
        },
        "max": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "Maximum amount of currency that can be requested"
        },
        "methods": {
          "type": "array",
          "items": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "type": "string",
                "description": "The type of payment method. e.g. BITCOIN_ADDRESS, DEBIT_CARD, etc."
              },
              "name": {
                "type": "string",
                "description": "Payment Method name. Expected to be rendered on screen."
              },
              "description": {
                "type": "string",
                "description": "Blurb containing helpful information about the payment method. Expected to be rendered on screen. e.g. \"segwit addresses only\""
              },
              "group": {
                "type": "string",
                "description": "Value that can be used to group specific payment methods together (e.g. Mobile Money vs. Direct Bank Deposit)."
              },
              "requiredPaymentDetails": {
                "$ref": "http://json-schema.org/draft-07/schema\#",
                "description": "A JSON Schema containing the fields that need to be collected in order to use this payment method"
              },
              "min": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Minimum amount required to use this payment method."
              },
              "max": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Maximum amount allowed when using this payment method."
              },
              "fee": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Fee charged to use this payment method. Absence of this field implies that there is no _additional_ fee associated to the respective payment method."
              }
            },
            "required": ["kind"]
          }
        }
      },
      "required": ["currencyCode", "methods"]
    },
    "payout": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "currencyCode": {
          "type": "string",
          "description": "ISO 4217 currency code string"
        },
        "min": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "Minimum amount of currency that can be requested"
        },
        "max": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "Maximum amount of currency that can be requested"
        },
        "methods": {
          "type": "array",
          "items": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "type": "string",
                "description": "The type of payment method. e.g. BITCOIN_ADDRESS, DEBIT_CARD, etc."
              },
              "name": {
                "type": "string",
                "description": "Payment Method name. Expected to be rendered on screen."
              },
              "description": {
                "type": "string",
                "description": "Blurb containing helpful information about the payment method. Expected to be rendered on screen. e.g. \"segwit addresses only\""
              },
              "group": {
                "type": "string",
                "description": "Value that can be used to group specific payment methods together (e.g. Mobile Money vs. Direct Bank Deposit)."
              },
              "requiredPaymentDetails": {
                "$ref": "http://json-schema.org/draft-07/schema\#",
                "description": "A JSON Schema containing the fields that need to be collected in order to use this payment method"
              },
              "min": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Minimum amount required to use this payment method."
              },
              "max": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Maximum amount allowed when using this payment method."
              },
              "fee": {
                "$ref": "definitions.json\#/definitions/decimalString",
                "description": "Fee charged to use this payment method. absence of this field implies that there is no _additional_ fee associated to the respective payment method"
              },
              "estimatedSettlementTime": {
                "type": "number",
                "description": "Estimated time in seconds for the payout to be settled. e.g. 3600 for 1 hour. 0 for instant settlement.",
                "minimum": 0
              }
            },
            "required": ["kind", "estimatedSettlementTime"]
          }
        }
      },
      "required": ["currencyCode", "methods"]
    },
    "payoutUnitsPerPayinUnit": {
      "type": "string",
      "description": "Number of payout currency units for one payin currency unit (i.e 290000 USD for 1 BTC)"
    },
    "requiredClaims": {
      "type": "object",
      "description": "PresentationDefinition that describes the credential(s) the PFI requires in order to provide a quote."
    }
  },
  "required": [
    "description",
    "payin",
    "payout",
    "payoutUnitsPerPayinUnit"
  ]
}
"#;
pub const MESSAGE_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/message.schema.json",
  "definitions": {
    "MessageMetadata": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "from": {
          "$ref": "definitions.json\#/definitions/did",
          "description": "The sender's DID"
        },
        "to": {
          "$ref": "definitions.json\#/definitions/did",
          "description": "The recipient's DID"
        },
        "kind": {
          "type": "string",
          "enum": ["rfq", "quote", "order", "orderstatus", "close"],
          "description": "The message kind (e.g. rfq, quote)"
        },
        "id": {
          "type": "string",
          "description": "The message ID"
        },
        "exchangeId": {
          "type": "string",
          "description": "ID for a 'thread' of messages between Alice <-> PFI. Set by the first message in a thread"
        },
        "externalId": {
          "type": "string",
          "description": "Arbitrary ID for the caller to associate with the message."
        },
        "createdAt": {
          "type": "string",
          "description": "ISO8601 formatted string representing the timestamp"
        },
        "protocol": {
          "type": "string",
          "description": "Version of the protocol in use (x.x format)"
        }
      },
      "required": ["from", "to", "kind", "id", "exchangeId", "createdAt", "protocol"]
    }
  },
  "type": "object",
  "properties": {
    "metadata": {
      "$ref": "\#/definitions/MessageMetadata"
    },
    "data": {
      "type": "object",
      "description": "The actual message content"
    },
    "signature": {
      "type": "string",
      "description": "Signature that verifies the authenticity and integrity of a message"
    },
    "privateData": {
      "type": "object",
      "description": "Private data which can be detached from the payload without disrupting integrity. Only used in RFQs"
    }
  },
  "additionalProperties": false,
  "required": ["metadata", "data", "signature"]
}
"#;
pub const RFQ_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/rfq.schema.json",
  "type": "object",
  "properties": {
    "additionalProperties": false,
    "offeringId": {
      "type": "string",
      "description": "Offering which Alice would like to get a quote for"
    },
    "claimsHash": {
      "type": "string",
      "description": "Digests of Presentation Submissions that fulfills the requirements included in the respective Offering"
    },
    "payin": {
      "type": "object",
      "properties": {
        "amount": {
          "$ref": "definitions.json\#/definitions/decimalString"
        },
        "kind": {
          "type": "string",
          "description": "Type of payment method e.g. BTC_ADDRESS, DEBIT_CARD, MOMO_MPESA"
        },
        "paymentDetailsHash": {
          "type": "string",
          "description": "Digest of an object containing the properties defined in the respective Offering's requiredPaymentDetails json schema"
        }
      },
      "required": ["amount", "kind"]
    },
    "payout": {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "description": "Selected payout method from the respective offering"
        },
        "paymentDetailsHash": {
          "type": "string",
          "description": "Digest of an object containing the properties defined in the respective Offering's requiredPaymentDetails json schema"
        }
      },
      "required": ["kind"]
    }
  },
  "required": ["offeringId", "payin", "payout"]
}
"#;
pub const RFQ_PRIVATE_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/rfq-private.schema.json",
  "type": "object",
  "properties": {
    "additionalProperties": false,
    "salt": {
      "type": "string",
      "description": "Randomly generated cryptographic salt used to hash privateData fields"
    },
    "claims": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "string"
      },
      "description": "Presentation Submission that fulfills the requirements included in the respective Offering"
    },
    "payin": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "paymentDetails": {
          "type": "object",
          "description": "An object containing the properties defined in the respective Offering's requiredPaymentDetails json schema"
        }
      }
    },
    "payout": {
      "additionalProperties": false,
      "type": "object",
      "properties": {
        "paymentDetails": {
          "type": "object",
          "description": "An object containing the properties defined in the respective Offering's requiredPaymentDetails json schema"
        }
      }
    }
  },
  "required": ["salt"]
}
"#;
pub const QUOTE_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/quote.schema.json",
  "definitions": {
    "QuoteDetails": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "currencyCode": {
          "type": "string",
          "description": "ISO 4217 currency code string"
        },
        "amount": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "The amount of currency expressed in the smallest respective unit"
        },
        "fee": {
          "$ref": "definitions.json\#/definitions/decimalString",
          "description": "The amount paid in fees"
        },
        "paymentInstruction": {
          "$ref": "\#/definitions/PaymentInstruction"
        }
      },
      "required": ["currencyCode", "amount"]
    },
    "PaymentInstruction": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "link": {
          "type": "string",
          "description": "Link to allow Alice to pay PFI, or be paid by the PFI"
        },
        "instruction": {
          "type": "string",
          "description": "Instruction on how Alice can pay PFI, or how Alice can be paid by the PFI"
        }
      }
    }
  },
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "expiresAt": {
      "type": "string",
      "description": "When this quote expires. Expressed as ISO8601"
    },
    "payin": {
      "$ref": "\#/definitions/QuoteDetails"
    },
    "payout": {
      "$ref": "\#/definitions/QuoteDetails"
    }
  },
  "required": ["expiresAt", "payin", "payout"]
}
"#;
pub const ORDER_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/order.schema.json",
  "type": "object",
  "additionalProperties": false,
  "properties": {}
}"#;
pub const ORDER_DATA_STATUS_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/orderstatus.schema.json",
  "type": "object",
  "required": [
    "orderStatus"
  ],
  "additionalProperties": false,
  "properties": {
    "orderStatus": {
      "type":"string"
    }
  }
}"#;
pub const CLOSE_DATA_JSON_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema\#",
  "$id": "https://tbdex.dev/close.schema.json",
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "reason": {
      "type": "string"
    },
    "success": {
      "type": "boolean"
    }
  }
}"#;
