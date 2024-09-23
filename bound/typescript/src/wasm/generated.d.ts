/* tslint:disable */
/* eslint-disable */
/**
*/
export class WasmBalance {
  free(): void;
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
}
/**
*/
export class WasmBearerDid {
  free(): void;
/**
* @param {WasmPortableDid} portable_did
* @returns {WasmBearerDid}
*/
  static from_portable_did(portable_did: WasmPortableDid): WasmBearerDid;
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
export class WasmGetBalancesResponseBody {
  free(): void;
/**
* @param {(WasmBalance)[]} data
*/
  constructor(data: (WasmBalance)[]);
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
export class WasmOffering {
  free(): void;
/**
* @param {WasmResourceMetadata} metadata
* @param {WasmOfferingData} data
* @param {string} signature
*/
  constructor(metadata: WasmResourceMetadata, data: WasmOfferingData, signature: string);
/**
* @param {string} from
* @param {WasmOfferingData} data
* @param {string | undefined} [protocol]
* @returns {WasmOffering}
*/
  static create(from: string, data: WasmOfferingData, protocol?: string): WasmOffering;
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
* @param {(WasmJwk)[]} private_jwks
*/
  constructor(did_uri: string, document: WasmDocument, private_jwks: (WasmJwk)[]);
/**
* @param {string} json
* @returns {WasmPortableDid}
*/
  static from_json_string(json: string): WasmPortableDid;
/**
* @returns {string}
*/
  to_json_string(): string;
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
export class WasmResourceKind {
  free(): void;
/**
* @param {string} kind
*/
  constructor(kind: string);
/**
*/
  readonly kind: string;
}
/**
*/
export class WasmResourceMetadata {
  free(): void;
/**
* @param {WasmResourceKind} kind
* @param {string} from
* @param {string} id
* @param {string} protocol
* @param {string} created_at
* @param {string | undefined} [updated_at]
*/
  constructor(kind: WasmResourceKind, from: string, id: string, protocol: string, created_at: string, updated_at?: string);
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
  readonly kind: WasmResourceKind;
/**
*/
  readonly protocol: string;
/**
*/
  readonly updated_at: string | undefined;
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