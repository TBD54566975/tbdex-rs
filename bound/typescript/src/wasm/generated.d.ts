/* tslint:disable */
/* eslint-disable */
/**
* @param {{ fetch: (url: string, options?: WasmFetchOptions) => WasmResponse }} foreign_fetch
*/
export function set_http_client(foreign_fetch: { fetch: (url: string, options?: WasmFetchOptions) => WasmResponse }): void;
/**
*/
export class WasmBalance {
  free(): void;
/**
* @param {WasmResourceMetadata} metadata
* @param {WasmBalanceData} data
* @param {string} signature
*/
  constructor(metadata: WasmResourceMetadata, data: WasmBalanceData, signature: string);
/**
* @param {string} json
* @returns {WasmBalance}
*/
  static from_json_string(json: string): WasmBalance;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} from
* @param {WasmBalanceData} data
* @param {string | undefined} [protocol]
* @returns {WasmBalance}
*/
  static create(from: string, data: WasmBalanceData, protocol?: string): WasmBalance;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmBalanceData;
/**
*/
  readonly metadata: WasmResourceMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmBalanceData {
  free(): void;
/**
* @param {string} currency_code
* @param {string} available
*/
  constructor(currency_code: string, available: string);
/**
*/
  readonly available: string;
/**
*/
  readonly currency_code: string;
}
/**
*/
export class WasmBearerDid {
  free(): void;
/**
* @param {WasmDid} did
* @param {WasmDocument} document
* @param {WasmKeyManager} key_manager
*/
  constructor(did: WasmDid, document: WasmDocument, key_manager: WasmKeyManager);
/**
* @param {WasmPortableDid} portable_did
* @returns {WasmBearerDid}
*/
  static from_portable_did(portable_did: WasmPortableDid): WasmBearerDid;
/**
* @param {string} verification_method_id
* @returns {WasmSigner}
*/
  get_signer(verification_method_id: string): WasmSigner;
/**
*/
  readonly did: WasmDid;
/**
*/
  readonly document: WasmDocument;
/**
*/
  readonly key_manager: WasmKeyManager;
}
/**
*/
export class WasmCancel {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmCancelData} data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmCancelData, signature: string);
/**
* @param {string} json
* @returns {WasmCancel}
*/
  static from_json_string(json: string): WasmCancel;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {string} exchange_id
* @param {WasmCancelData} data
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmCancel}
*/
  static create(to: string, from: string, exchange_id: string, data: WasmCancelData, protocol?: string, external_id?: string): WasmCancel;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmCancelData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmCancelData {
  free(): void;
/**
* @param {string | undefined} [reason]
*/
  constructor(reason?: string);
/**
*/
  readonly reason: string | undefined;
}
/**
*/
export class WasmCancellationDetails {
  free(): void;
/**
* @param {boolean} enabled
* @param {string | undefined} [terms_url]
* @param {string | undefined} [terms]
*/
  constructor(enabled: boolean, terms_url?: string, terms?: string);
/**
*/
  readonly enabled: boolean;
/**
*/
  readonly terms: string | undefined;
/**
*/
  readonly terms_url: string | undefined;
}
/**
*/
export class WasmConstraints {
  free(): void;
/**
* @param {(WasmField)[]} fields
*/
  constructor(fields: (WasmField)[]);
/**
*/
  readonly fields: (WasmField)[];
}
/**
*/
export class WasmCreateRfqData {
  free(): void;
/**
* @param {string} offering_id
* @param {WasmCreateSelectedPayinMethod} payin
* @param {WasmCreateSelectedPayoutMethod} payout
* @param {Array<any>} claims
*/
  constructor(offering_id: string, payin: WasmCreateSelectedPayinMethod, payout: WasmCreateSelectedPayoutMethod, claims: Array<any>);
/**
*/
  readonly claims: Array<any>;
/**
*/
  readonly offering_id: string;
/**
*/
  readonly payin: WasmCreateSelectedPayinMethod;
/**
*/
  readonly payout: WasmCreateSelectedPayoutMethod;
}
/**
*/
export class WasmCreateSelectedPayinMethod {
  free(): void;
/**
* @param {string} kind
* @param {any} payment_details
* @param {string} amount
*/
  constructor(kind: string, payment_details: any, amount: string);
/**
*/
  readonly amount: string;
/**
*/
  readonly kind: string;
/**
*/
  readonly payment_details: any;
}
/**
*/
export class WasmCreateSelectedPayoutMethod {
  free(): void;
/**
* @param {string} kind
* @param {any} payment_details
*/
  constructor(kind: string, payment_details: any);
/**
*/
  readonly kind: string;
/**
*/
  readonly payment_details: any;
}
/**
*/
export class WasmDid {
  free(): void;
/**
* @param {string} uri
* @param {string} url
* @param {string} method
* @param {string} id
* @param {any} params
* @param {string | undefined} [path]
* @param {string | undefined} [query]
* @param {string | undefined} [fragment]
*/
  constructor(uri: string, url: string, method: string, id: string, params: any, path?: string, query?: string, fragment?: string);
/**
*/
  readonly fragment: string | undefined;
/**
*/
  readonly id: string;
/**
*/
  readonly method: string;
/**
*/
  readonly params: any;
/**
*/
  readonly path: string | undefined;
/**
*/
  readonly query: string | undefined;
/**
*/
  readonly uri: string;
/**
*/
  readonly url: string;
}
/**
*/
export class WasmDocument {
  free(): void;
/**
* @param {string} id
* @param {(string)[] | undefined} context
* @param {(string)[] | undefined} controller
* @param {(string)[] | undefined} also_known_as
* @param {(WasmVerificationMethod)[]} verification_method
* @param {(string)[] | undefined} [authentication]
* @param {(string)[] | undefined} [assertion_method]
* @param {(string)[] | undefined} [key_agreement]
* @param {(string)[] | undefined} [capability_invocation]
* @param {(string)[] | undefined} [capability_delegation]
* @param {(WasmService)[] | undefined} [service]
*/
  constructor(id: string, context: (string)[] | undefined, controller: (string)[] | undefined, also_known_as: (string)[] | undefined, verification_method: (WasmVerificationMethod)[], authentication?: (string)[], assertion_method?: (string)[], key_agreement?: (string)[], capability_invocation?: (string)[], capability_delegation?: (string)[], service?: (WasmService)[]);
/**
* @param {string} json
* @returns {WasmDocument}
*/
  static from_json_string(json: string): WasmDocument;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
*/
  readonly also_known_as: (string)[] | undefined;
/**
*/
  readonly assertion_method: (string)[] | undefined;
/**
*/
  readonly authentication: (string)[] | undefined;
/**
*/
  readonly capability_delegation: (string)[] | undefined;
/**
*/
  readonly capability_invocation: (string)[] | undefined;
/**
*/
  readonly context: (string)[] | undefined;
/**
*/
  readonly controller: (string)[] | undefined;
/**
*/
  readonly id: string;
/**
*/
  readonly key_agreement: (string)[] | undefined;
/**
*/
  readonly service: (WasmService)[] | undefined;
/**
*/
  readonly verification_method: (WasmVerificationMethod)[];
}
/**
*/
export class WasmFetchOptions {
  free(): void;
/**
* @param {string | undefined} method
* @param {any} headers
* @param {Uint8Array | undefined} [body]
*/
  constructor(method: string | undefined, headers: any, body?: Uint8Array);
/**
*/
  readonly body: Uint8Array | undefined;
/**
*/
  readonly headers: any;
/**
*/
  readonly method: string | undefined;
}
/**
*/
export class WasmField {
  free(): void;
/**
* @param {string | undefined} id
* @param {string | undefined} name
* @param {(string)[]} path
* @param {string | undefined} [purpose]
* @param {WasmFilter | undefined} [filter]
* @param {boolean | undefined} [optional]
* @param {WasmOptionality | undefined} [predicate]
*/
  constructor(id: string | undefined, name: string | undefined, path: (string)[], purpose?: string, filter?: WasmFilter, optional?: boolean, predicate?: WasmOptionality);
/**
*/
  readonly filter: WasmFilter | undefined;
/**
*/
  readonly id: string | undefined;
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly optional: boolean | undefined;
/**
*/
  readonly path: (string)[];
/**
*/
  readonly predicate: WasmOptionality | undefined;
/**
*/
  readonly purpose: string | undefined;
}
/**
*/
export class WasmFilter {
  free(): void;
/**
* @param {string | undefined} [type]
* @param {string | undefined} [pattern]
* @param {string | undefined} [const_value]
* @param {WasmFilter | undefined} [contains]
*/
  constructor(type?: string, pattern?: string, const_value?: string, contains?: WasmFilter);
/**
*/
  readonly const_value: string | undefined;
/**
*/
  readonly contains: WasmFilter | undefined;
/**
*/
  readonly pattern: string | undefined;
/**
*/
  readonly type: string | undefined;
}
/**
*/
export class WasmInputDescriptor {
  free(): void;
/**
* @param {string} id
* @param {string | undefined} name
* @param {string | undefined} purpose
* @param {WasmConstraints} constraints
*/
  constructor(id: string, name: string | undefined, purpose: string | undefined, constraints: WasmConstraints);
/**
*/
  readonly constraints: WasmConstraints;
/**
*/
  readonly id: string;
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly purpose: string | undefined;
}
/**
*/
export class WasmJwk {
  free(): void;
/**
* @param {string | undefined} alg
* @param {string} kty
* @param {string} crv
* @param {string | undefined} d
* @param {string} x
* @param {string | undefined} [y]
*/
  constructor(alg: string | undefined, kty: string, crv: string, d: string | undefined, x: string, y?: string);
/**
* @returns {string}
*/
  compute_thumbprint(): string;
/**
*/
  readonly alg: string | undefined;
/**
*/
  readonly crv: string;
/**
*/
  readonly d: string | undefined;
/**
*/
  readonly kty: string;
/**
*/
  readonly x: string;
/**
*/
  readonly y: string | undefined;
}
/**
*/
export class WasmKeyManager {
  free(): void;
/**
* @param {{ import_private_jwk: (private_jwk: WasmJwk) => WasmJwk, get_signer: (public_jwk: WasmJwk) => WasmSigner }} foreign_key_manager
*/
  constructor(foreign_key_manager: { import_private_jwk: (private_jwk: WasmJwk) => WasmJwk, get_signer: (public_jwk: WasmJwk) => WasmSigner });
/**
* @param {WasmJwk} private_jwk
* @returns {WasmJwk}
*/
  import_private_jwk(private_jwk: WasmJwk): WasmJwk;
/**
* @param {WasmJwk} public_jwk
* @returns {WasmSigner}
*/
  get_signer(public_jwk: WasmJwk): WasmSigner;
}
/**
*/
export class WasmMessageMetadata {
  free(): void;
/**
* @param {string} from
* @param {string} to
* @param {string} kind
* @param {string} id
* @param {string} exchange_id
* @param {string | undefined} external_id
* @param {string} protocol
* @param {string} created_at
*/
  constructor(from: string, to: string, kind: string, id: string, exchange_id: string, external_id: string | undefined, protocol: string, created_at: string);
/**
*/
  readonly created_at: string;
/**
*/
  readonly exchange_id: string;
/**
*/
  readonly external_id: string | undefined;
/**
*/
  readonly from: string;
/**
*/
  readonly id: string;
/**
*/
  readonly kind: string;
/**
*/
  readonly protocol: string;
/**
*/
  readonly to: string;
}
/**
*/
export class WasmOffering {
  free(): void;
/**
* @param {WasmResourceMetadata} metadata
* @param {WasmOfferingData} data
* @param {string} signature
*/
  constructor(metadata: WasmResourceMetadata, data: WasmOfferingData, signature: string);
/**
* @param {string} json
* @returns {WasmOffering}
*/
  static from_json_string(json: string): WasmOffering;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} from
* @param {WasmOfferingData} data
* @param {string | undefined} [protocol]
* @returns {WasmOffering}
*/
  static create(from: string, data: WasmOfferingData, protocol?: string): WasmOffering;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmOfferingData;
/**
*/
  readonly metadata: WasmResourceMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmOfferingData {
  free(): void;
/**
* @param {string} description
* @param {string} payout_units_per_payin_unit
* @param {WasmPayinDetails} payin
* @param {WasmPayoutDetails} payout
* @param {WasmPresentationDefinition | undefined} required_claims
* @param {WasmCancellationDetails} cancellation
*/
  constructor(description: string, payout_units_per_payin_unit: string, payin: WasmPayinDetails, payout: WasmPayoutDetails, required_claims: WasmPresentationDefinition | undefined, cancellation: WasmCancellationDetails);
/**
*/
  readonly cancellation: WasmCancellationDetails;
/**
*/
  readonly description: string;
/**
*/
  readonly payin: WasmPayinDetails;
/**
*/
  readonly payout: WasmPayoutDetails;
/**
*/
  readonly payout_units_per_payin_unit: string;
/**
*/
  readonly required_claims: WasmPresentationDefinition | undefined;
}
/**
*/
export class WasmOptionality {
  free(): void;
/**
* @param {string} optionality
*/
  constructor(optionality: string);
/**
*/
  readonly optionality: string;
}
/**
*/
export class WasmOrder {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmOrderData} data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmOrderData, signature: string);
/**
* @param {string} json
* @returns {WasmOrder}
*/
  static from_json_string(json: string): WasmOrder;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {string} exchange_id
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmOrder}
*/
  static create(to: string, from: string, exchange_id: string, protocol?: string, external_id?: string): WasmOrder;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmOrderData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmOrderData {
  free(): void;
/**
*/
  constructor();
}
/**
*/
export class WasmOrderInstructions {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmOrderInstructionsData} data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmOrderInstructionsData, signature: string);
/**
* @param {string} json
* @returns {WasmOrderInstructions}
*/
  static from_json_string(json: string): WasmOrderInstructions;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {string} exchange_id
* @param {WasmOrderInstructionsData} data
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmOrderInstructions}
*/
  static create(to: string, from: string, exchange_id: string, data: WasmOrderInstructionsData, protocol?: string, external_id?: string): WasmOrderInstructions;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmOrderInstructionsData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmOrderInstructionsData {
  free(): void;
/**
* @param {WasmPaymentInstruction} payin
* @param {WasmPaymentInstruction} payout
*/
  constructor(payin: WasmPaymentInstruction, payout: WasmPaymentInstruction);
/**
*/
  readonly payin: WasmPaymentInstruction;
/**
*/
  readonly payout: WasmPaymentInstruction;
}
/**
*/
export class WasmOrderStatus {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmOrderStatusData} data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmOrderStatusData, signature: string);
/**
* @param {string} json
* @returns {WasmOrderStatus}
*/
  static from_json_string(json: string): WasmOrderStatus;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {string} exchange_id
* @param {WasmOrderStatusData} data
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmOrderStatus}
*/
  static create(to: string, from: string, exchange_id: string, data: WasmOrderStatusData, protocol?: string, external_id?: string): WasmOrderStatus;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmOrderStatusData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmOrderStatusData {
  free(): void;
/**
* @param {string} status
* @param {string | undefined} [details]
*/
  constructor(status: string, details?: string);
/**
*/
  readonly details: string | undefined;
/**
*/
  readonly status: string;
}
/**
*/
export class WasmPayinDetails {
  free(): void;
/**
* @param {string} currency_code
* @param {(WasmPayinMethod)[]} methods
* @param {string | undefined} [min]
* @param {string | undefined} [max]
*/
  constructor(currency_code: string, methods: (WasmPayinMethod)[], min?: string, max?: string);
/**
*/
  readonly currency_code: string;
/**
*/
  readonly max: string | undefined;
/**
*/
  readonly methods: (WasmPayinMethod)[];
/**
*/
  readonly min: string | undefined;
}
/**
*/
export class WasmPayinMethod {
  free(): void;
/**
* @param {string} kind
* @param {string | undefined} name
* @param {string | undefined} description
* @param {string | undefined} group
* @param {any} required_payment_details
* @param {string | undefined} [fee]
* @param {string | undefined} [min]
* @param {string | undefined} [max]
*/
  constructor(kind: string, name: string | undefined, description: string | undefined, group: string | undefined, required_payment_details: any, fee?: string, min?: string, max?: string);
/**
*/
  readonly description: string | undefined;
/**
*/
  readonly fee: string | undefined;
/**
*/
  readonly group: string | undefined;
/**
*/
  readonly kind: string;
/**
*/
  readonly max: string | undefined;
/**
*/
  readonly min: string | undefined;
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly required_payment_details: any;
}
/**
*/
export class WasmPaymentInstruction {
  free(): void;
/**
* @param {string | undefined} [link]
* @param {string | undefined} [instruction]
*/
  constructor(link?: string, instruction?: string);
/**
*/
  readonly instruction: string | undefined;
/**
*/
  readonly link: string | undefined;
}
/**
*/
export class WasmPayoutDetails {
  free(): void;
/**
* @param {string} currency_code
* @param {(WasmPayoutMethod)[]} methods
* @param {string | undefined} [min]
* @param {string | undefined} [max]
*/
  constructor(currency_code: string, methods: (WasmPayoutMethod)[], min?: string, max?: string);
/**
*/
  readonly currency_code: string;
/**
*/
  readonly max: string | undefined;
/**
*/
  readonly methods: (WasmPayoutMethod)[];
/**
*/
  readonly min: string | undefined;
}
/**
*/
export class WasmPayoutMethod {
  free(): void;
/**
* @param {string} kind
* @param {bigint} estimated_settlement_time
* @param {string | undefined} name
* @param {string | undefined} description
* @param {string | undefined} group
* @param {any} required_payment_details
* @param {string | undefined} [fee]
* @param {string | undefined} [min]
* @param {string | undefined} [max]
*/
  constructor(kind: string, estimated_settlement_time: bigint, name: string | undefined, description: string | undefined, group: string | undefined, required_payment_details: any, fee?: string, min?: string, max?: string);
/**
*/
  readonly description: string | undefined;
/**
*/
  readonly estimated_settlement_time: bigint;
/**
*/
  readonly fee: string | undefined;
/**
*/
  readonly group: string | undefined;
/**
*/
  readonly kind: string;
/**
*/
  readonly max: string | undefined;
/**
*/
  readonly min: string | undefined;
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly required_payment_details: any;
}
/**
*/
export class WasmPortableDid {
  free(): void;
/**
* @param {string} did_uri
* @param {WasmDocument} document
* @param {(WasmJwk)[]} private_keys
*/
  constructor(did_uri: string, document: WasmDocument, private_keys: (WasmJwk)[]);
/**
* @param {string} json
* @returns {WasmPortableDid}
*/
  static from_json_string(json: string): WasmPortableDid;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
*/
  readonly did_uri: string;
/**
*/
  readonly document: WasmDocument;
/**
*/
  readonly private_keys: (WasmJwk)[];
}
/**
*/
export class WasmPresentationDefinition {
  free(): void;
/**
* @param {string} id
* @param {string | undefined} name
* @param {string | undefined} purpose
* @param {(WasmInputDescriptor)[]} input_descriptors
* @param {(WasmSubmissionRequirement)[] | undefined} [submission_requirements]
*/
  constructor(id: string, name: string | undefined, purpose: string | undefined, input_descriptors: (WasmInputDescriptor)[], submission_requirements?: (WasmSubmissionRequirement)[]);
/**
*/
  readonly id: string;
/**
*/
  readonly input_descriptors: (WasmInputDescriptor)[];
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly purpose: string | undefined;
/**
*/
  readonly submission_requirements: (WasmSubmissionRequirement)[] | undefined;
}
/**
*/
export class WasmPrivatePaymentDetails {
  free(): void;
/**
* @param {any} payment_details
*/
  constructor(payment_details: any);
/**
*/
  readonly payment_details: any;
}
/**
*/
export class WasmQuote {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmQuoteData} data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmQuoteData, signature: string);
/**
* @param {string} json
* @returns {WasmQuote}
*/
  static from_json_string(json: string): WasmQuote;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {string} exchange_id
* @param {WasmQuoteData} data
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmQuote}
*/
  static create(to: string, from: string, exchange_id: string, data: WasmQuoteData, protocol?: string, external_id?: string): WasmQuote;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
*/
  readonly data: WasmQuoteData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmQuoteData {
  free(): void;
/**
* @param {string} expires_at
* @param {string} payout_units_per_payin_unit
* @param {WasmQuoteDetails} payin
* @param {WasmQuoteDetails} payout
*/
  constructor(expires_at: string, payout_units_per_payin_unit: string, payin: WasmQuoteDetails, payout: WasmQuoteDetails);
/**
*/
  readonly expires_at: string;
/**
*/
  readonly payin: WasmQuoteDetails;
/**
*/
  readonly payout: WasmQuoteDetails;
/**
*/
  readonly payout_units_per_payin_unit: string;
}
/**
*/
export class WasmQuoteDetails {
  free(): void;
/**
* @param {string} currency_code
* @param {string} subtotal
* @param {string} total
* @param {string | undefined} [fee]
*/
  constructor(currency_code: string, subtotal: string, total: string, fee?: string);
/**
*/
  readonly currency_code: string;
/**
*/
  readonly fee: string | undefined;
/**
*/
  readonly subtotal: string;
/**
*/
  readonly total: string;
}
/**
*/
export class WasmResourceMetadata {
  free(): void;
/**
* @param {string} kind
* @param {string} from
* @param {string} id
* @param {string} protocol
* @param {string} created_at
* @param {string | undefined} [updated_at]
*/
  constructor(kind: string, from: string, id: string, protocol: string, created_at: string, updated_at?: string);
/**
*/
  readonly created_at: string;
/**
*/
  readonly from: string;
/**
*/
  readonly id: string;
/**
*/
  readonly kind: string;
/**
*/
  readonly protocol: string;
/**
*/
  readonly updated_at: string | undefined;
}
/**
*/
export class WasmResponse {
  free(): void;
/**
* @param {number} status_code
* @param {any} headers
* @param {Uint8Array} body
*/
  constructor(status_code: number, headers: any, body: Uint8Array);
/**
*/
  readonly body: Uint8Array;
/**
*/
  readonly headers: any;
/**
*/
  readonly status_code: number;
}
/**
*/
export class WasmRfq {
  free(): void;
/**
* @param {WasmMessageMetadata} metadata
* @param {WasmRfqData} data
* @param {WasmRfqPrivateData | undefined} private_data
* @param {string} signature
*/
  constructor(metadata: WasmMessageMetadata, data: WasmRfqData, private_data: WasmRfqPrivateData | undefined, signature: string);
/**
* @param {string} json
* @returns {WasmRfq}
*/
  static from_json_string(json: string): WasmRfq;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} to
* @param {string} from
* @param {WasmCreateRfqData} create_rfq_data
* @param {string | undefined} [protocol]
* @param {string | undefined} [external_id]
* @returns {WasmRfq}
*/
  static create(to: string, from: string, create_rfq_data: WasmCreateRfqData, protocol?: string, external_id?: string): WasmRfq;
/**
* @param {WasmBearerDid} bearer_did
*/
  sign(bearer_did: WasmBearerDid): void;
/**
*/
  verify(): void;
/**
* @param {WasmOffering} offering
*/
  verify_offering_requirements(offering: WasmOffering): void;
/**
*/
  verify_all_private_data(): void;
/**
*/
  verify_present_private_data(): void;
/**
*/
  readonly data: WasmRfqData;
/**
*/
  readonly metadata: WasmMessageMetadata;
/**
*/
  readonly private_data: WasmRfqPrivateData | undefined;
/**
*/
  readonly signature: string;
}
/**
*/
export class WasmRfqData {
  free(): void;
/**
* @param {string} offering_id
* @param {WasmSelectedPayinMethod} payin
* @param {WasmSelectedPayoutMethod} payout
* @param {string | undefined} [claims_hash]
*/
  constructor(offering_id: string, payin: WasmSelectedPayinMethod, payout: WasmSelectedPayoutMethod, claims_hash?: string);
/**
*/
  readonly claims_hash: string | undefined;
/**
*/
  readonly offering_id: string;
/**
*/
  readonly payin: WasmSelectedPayinMethod;
/**
*/
  readonly payout: WasmSelectedPayoutMethod;
}
/**
*/
export class WasmRfqPrivateData {
  free(): void;
/**
* @param {string} salt
* @param {WasmPrivatePaymentDetails | undefined} [payin]
* @param {WasmPrivatePaymentDetails | undefined} [payout]
* @param {Array<any> | undefined} [claims]
*/
  constructor(salt: string, payin?: WasmPrivatePaymentDetails, payout?: WasmPrivatePaymentDetails, claims?: Array<any>);
/**
*/
  readonly claims: Array<any> | undefined;
/**
*/
  readonly payin: WasmPrivatePaymentDetails | undefined;
/**
*/
  readonly payout: WasmPrivatePaymentDetails | undefined;
/**
*/
  readonly salt: string;
}
/**
*/
export class WasmSelectedPayinMethod {
  free(): void;
/**
* @param {string} kind
* @param {string | undefined} payment_details_hash
* @param {string} amount
*/
  constructor(kind: string, payment_details_hash: string | undefined, amount: string);
/**
*/
  readonly amount: string;
/**
*/
  readonly kind: string;
/**
*/
  readonly payment_details_hash: string | undefined;
}
/**
*/
export class WasmSelectedPayoutMethod {
  free(): void;
/**
* @param {string} kind
* @param {string | undefined} [payment_details_hash]
*/
  constructor(kind: string, payment_details_hash?: string);
/**
*/
  readonly kind: string;
/**
*/
  readonly payment_details_hash: string | undefined;
}
/**
*/
export class WasmService {
  free(): void;
/**
* @param {string} id
* @param {string} type
* @param {(string)[]} service_endpoint
*/
  constructor(id: string, type: string, service_endpoint: (string)[]);
/**
*/
  readonly id: string;
/**
*/
  readonly service_endpoint: (string)[];
/**
*/
  readonly type: string;
}
/**
*/
export class WasmSigner {
  free(): void;
/**
* @param {{ sign: (payload: Uint8Array) => Uint8Array }} foreign_signer
*/
  constructor(foreign_signer: { sign: (payload: Uint8Array) => Uint8Array });
/**
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
  sign(payload: Uint8Array): Uint8Array;
}
/**
*/
export class WasmSubmissionRequirement {
  free(): void;
/**
* @param {WasmSubmissionRequirementRule} rule
* @param {string | undefined} [from]
* @param {(WasmSubmissionRequirement)[] | undefined} [from_nested]
* @param {string | undefined} [name]
* @param {string | undefined} [purpose]
* @param {number | undefined} [count]
* @param {number | undefined} [min]
* @param {number | undefined} [max]
*/
  constructor(rule: WasmSubmissionRequirementRule, from?: string, from_nested?: (WasmSubmissionRequirement)[], name?: string, purpose?: string, count?: number, min?: number, max?: number);
/**
*/
  readonly count: number | undefined;
/**
*/
  readonly from: string | undefined;
/**
*/
  readonly from_nested: (WasmSubmissionRequirement)[] | undefined;
/**
*/
  readonly max: number | undefined;
/**
*/
  readonly min: number | undefined;
/**
*/
  readonly name: string | undefined;
/**
*/
  readonly purpose: string | undefined;
/**
*/
  readonly rule: WasmSubmissionRequirementRule;
}
/**
*/
export class WasmSubmissionRequirementRule {
  free(): void;
/**
* @param {string} rule
*/
  constructor(rule: string);
/**
*/
  readonly rule: string;
}
/**
*/
export class WasmTbdexError {
  free(): void;
/**
*/
  readonly is_web5_error: boolean;
/**
*/
  readonly message: string;
/**
*/
  readonly variant: string;
}
/**
*/
export class WasmVerificationMethod {
  free(): void;
/**
* @param {string} id
* @param {string} type
* @param {string} controller
* @param {WasmJwk} public_key_jwk
*/
  constructor(id: string, type: string, controller: string, public_key_jwk: WasmJwk);
/**
*/
  readonly controller: string;
/**
*/
  readonly id: string;
/**
*/
  readonly public_key_jwk: WasmJwk;
/**
*/
  readonly type: string;
}
/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
export function loadWasmAsync(): Promise<void>;

export function loadWasmSync(): void;