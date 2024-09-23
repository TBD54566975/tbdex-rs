import {
  CancellationDetails,
  OfferingData,
  PayinDetails,
  PayinMethod,
  PayoutDetails,
  PayoutMethod,
} from "../resources/offering";
import {
  Constraints,
  Field,
  Filter,
  InputDescriptor,
  PresentationDefinition,
  SubmissionRequirement,
} from "../web5/presentation-definition";
import wasm from "./generated";
export { default } from "./generated";

wasm.loadWasmSync();

// export const offeringDataToWasm = (
//   offeringData: OfferingData
// ): wasm.WasmOfferingData => {
//   return new wasm.WasmOfferingData(
//     offeringData.description,
//     offeringData.payoutUnitsPerPayinUnit,
//     new wasm.WasmPayinDetails(
//       offeringData.payin.currencyCode,
//       offeringData.payin.methods.map(
//         (method) =>
//           new wasm.WasmPayinMethod(
//             method.kind,
//             method.name,
//             method.description,
//             method.group,
//             method.requiredPaymentDetails,
//             method.fee,
//             method.min,
//             method.max
//           )
//       ),
//       offeringData.payin.min,
//       offeringData.payin.max
//     ),
//     new wasm.WasmPayoutDetails(
//       offeringData.payout.currencyCode,
//       offeringData.payout.methods.map(
//         (method) =>
//           new wasm.WasmPayoutMethod(
//             method.kind,
//             BigInt(method.estimatedSettlementTime),
//             method.name,
//             method.description,
//             method.group,
//             method.requiredPaymentDetails,
//             method.fee,
//             method.min,
//             method.max
//           )
//       ),
//       offeringData.payout.min,
//       offeringData.payout.max
//     ),
//     offeringData.requiredClaims
//       ? presentationDefinitionToWASM(offeringData.requiredClaims)
//       : undefined,
//     new wasm.WasmCancellationDetails(
//       offeringData.cancellation.enabled,
//       offeringData.cancellation.termsUrl,
//       offeringData.cancellation.terms
//     )
//   );
// };

// const mapToObject = (map: Map<any, any>): any => {
//   const obj: any = {};
//   for (const [key, value] of map) {
//     obj[key] = value instanceof Map ? mapToObject(value) : value;
//   }
//   return obj;
// };

// export const offeringDataFromWasm = (
//   wasmOfferingData: wasm.WasmOfferingData
// ): OfferingData => {
//   const payinMethods = wasmOfferingData.payin.methods.map((wasmMethod) => {
//     const method: PayinMethod = {
//       kind: wasmMethod.kind,
//       requiredPaymentDetails: mapToObject(wasmMethod.required_payment_details),
//     };

//     if (wasmMethod.name !== undefined) method.name = wasmMethod.name;
//     if (wasmMethod.description !== undefined)
//       method.description = wasmMethod.description;
//     if (wasmMethod.group !== undefined) method.group = wasmMethod.group;
//     if (wasmMethod.fee !== undefined) method.fee = wasmMethod.fee;
//     if (wasmMethod.min !== undefined) method.min = wasmMethod.min;
//     if (wasmMethod.max !== undefined) method.max = wasmMethod.max;

//     return method;
//   });

//   const payinDetails: PayinDetails = {
//     currencyCode: wasmOfferingData.payin.currency_code,
//     methods: payinMethods,
//   };

//   if (wasmOfferingData.payin.min !== undefined)
//     payinDetails.min = wasmOfferingData.payin.min;
//   if (wasmOfferingData.payin.max !== undefined)
//     payinDetails.max = wasmOfferingData.payin.max;

//   const payoutMethods = wasmOfferingData.payout.methods.map((wasmMethod) => {
//     const method: PayoutMethod = {
//       kind: wasmMethod.kind,
//       estimatedSettlementTime: Number(wasmMethod.estimated_settlement_time),
//     };

//     if (wasmMethod.name !== undefined) method.name = wasmMethod.name;
//     if (wasmMethod.description !== undefined)
//       method.description = wasmMethod.description;
//     if (wasmMethod.group !== undefined) method.group = wasmMethod.group;
//     if (wasmMethod.fee !== undefined) method.fee = wasmMethod.fee;
//     if (wasmMethod.min !== undefined) method.min = wasmMethod.min;
//     if (wasmMethod.max !== undefined) method.max = wasmMethod.max;

//     return method;
//   });

//   const payoutDetails: PayoutDetails = {
//     currencyCode: wasmOfferingData.payout.currency_code,
//     methods: payoutMethods,
//   };

//   if (wasmOfferingData.payout.min !== undefined)
//     payoutDetails.min = wasmOfferingData.payout.min;
//   if (wasmOfferingData.payout.max !== undefined)
//     payoutDetails.max = wasmOfferingData.payout.max;

//   const cancellationDetails: CancellationDetails = {
//     enabled: wasmOfferingData.cancellation.enabled,
//   };

//   if (wasmOfferingData.cancellation.terms_url !== undefined)
//     cancellationDetails.termsUrl = wasmOfferingData.cancellation.terms_url;
//   if (wasmOfferingData.cancellation.terms !== undefined)
//     cancellationDetails.terms = wasmOfferingData.cancellation.terms;

//   const offeringData: OfferingData = {
//     description: wasmOfferingData.description,
//     payoutUnitsPerPayinUnit: wasmOfferingData.payout_units_per_payin_unit,
//     payin: payinDetails,
//     payout: payoutDetails,
//     cancellation: cancellationDetails,
//   };

//   if (wasmOfferingData.required_claims)
//     offeringData.requiredClaims = presentationDefinitionFromWASM(
//       wasmOfferingData.required_claims
//     );

//   return offeringData;
// };

// export const presentationDefinitionToWASM = (
//   presentationDefinition: PresentationDefinition
// ): wasm.WasmPresentationDefinition => {
//   return new wasm.WasmPresentationDefinition(
//     presentationDefinition.id,
//     presentationDefinition.name,
//     presentationDefinition.purpose,
//     presentationDefinition.input_descriptors.map(
//       (desc) =>
//         new wasm.WasmInputDescriptor(
//           desc.id,
//           desc.name,
//           desc.purpose,
//           new wasm.WasmConstraints(
//             desc.constraints.fields.map((field) => {
//               const wasmField = new wasm.WasmField(
//                 field.id,
//                 field.name,
//                 field.path,
//                 field.purpose,
//                 field.filter
//                   ? new wasm.WasmFilter(
//                       field.filter.type,
//                       field.filter.pattern,
//                       field.filter.constValue,
//                       field.filter.contains
//                         ? new wasm.WasmFilter(
//                             field.filter.contains.type,
//                             field.filter.contains.pattern,
//                             field.filter.contains.constValue,
//                             field.filter.contains.contains
//                               ? new wasm.WasmFilter(
//                                   field.filter.contains.contains.type,
//                                   field.filter.contains.contains.pattern,
//                                   field.filter.contains.contains.constValue,
//                                   field.filter.contains.contains
//                                     ? filterToWasm(
//                                         field.filter.contains.contains
//                                       )
//                                     : undefined
//                                 )
//                               : undefined
//                           )
//                         : undefined
//                     )
//                   : undefined,
//                 field.optional,
//                 field.predicate
//                   ? new wasm.WasmOptionality(field.predicate.optionality)
//                   : undefined
//               );
//               return wasmField;
//             })
//           )
//         )
//     ),
//     presentationDefinition.submission_requirements?.map((req) =>
//       submissionRequirementToWASM(req)
//     )
//   );
// };

// const filterToWasm = (filter: Filter): wasm.WasmFilter => {
//   return new wasm.WasmFilter(
//     filter.type || undefined,
//     filter.pattern || undefined,
//     filter.constValue || undefined,
//     filter.contains ? filterToWasm(filter.contains) : undefined
//   );
// };

// const submissionRequirementToWASM = (
//   req: SubmissionRequirement
// ): wasm.WasmSubmissionRequirement => {
//   return new wasm.WasmSubmissionRequirement(
//     new wasm.WasmSubmissionRequirementRule(req.rule.rule),
//     req.from || undefined,
//     req.from_nested?.map((nestedReq) => submissionRequirementToWASM(nestedReq)),
//     req.name || undefined,
//     req.purpose || undefined,
//     req.count || undefined,
//     req.min || undefined,
//     req.max || undefined
//   );
// };

// export const presentationDefinitionFromWASM = (
//   wasmPresentationDefinition: wasm.WasmPresentationDefinition
// ): PresentationDefinition => {
//   const inputDescriptors: InputDescriptor[] =
//     wasmPresentationDefinition.input_descriptors.map((wasmDesc) => {
//       const fields: Field[] = wasmDesc.constraints.fields.map((wasmField) => {
//         const field: Field = {
//           path: wasmField.path,
//         };

//         if (wasmField.id !== undefined) field.id = wasmField.id;
//         if (wasmField.name !== undefined) field.name = wasmField.name;
//         if (wasmField.purpose !== undefined) field.purpose = wasmField.purpose;

//         if (wasmField.filter !== undefined) {
//           const filter: Filter = {};
//           if (wasmField.filter.type !== undefined)
//             filter.type = wasmField.filter.type;
//           if (wasmField.filter.pattern !== undefined)
//             filter.pattern = wasmField.filter.pattern;
//           if (wasmField.filter.const_value !== undefined)
//             filter.constValue = wasmField.filter.const_value;

//           if (wasmField.filter.contains !== undefined) {
//             const containsFilter: Filter = {};
//             if (wasmField.filter.contains.type !== undefined)
//               containsFilter.type = wasmField.filter.contains.type;
//             if (wasmField.filter.contains.pattern !== undefined)
//               containsFilter.pattern = wasmField.filter.contains.pattern;
//             if (wasmField.filter.contains.const_value !== undefined)
//               containsFilter.constValue = wasmField.filter.contains.const_value;

//             if (wasmField.filter.contains.contains !== undefined) {
//               const deepContainsFilter: Filter = {};
//               if (wasmField.filter.contains.contains.type !== undefined)
//                 deepContainsFilter.type =
//                   wasmField.filter.contains.contains.type;
//               if (wasmField.filter.contains.contains.pattern !== undefined)
//                 deepContainsFilter.pattern =
//                   wasmField.filter.contains.contains.pattern;
//               if (wasmField.filter.contains.contains.const_value !== undefined)
//                 deepContainsFilter.constValue =
//                   wasmField.filter.contains.contains.const_value;

//               containsFilter.contains = deepContainsFilter;
//             }

//             filter.contains = containsFilter;
//           }

//           field.filter = filter;
//         }

//         if (wasmField.optional !== undefined)
//           field.optional = wasmField.optional;

//         if (wasmField.predicate !== undefined) {
//           field.predicate = {
//             optionality: wasmField.predicate.optionality,
//           };
//         }

//         return field;
//       });

//       const constraints: Constraints = { fields };

//       const inputDescriptor: InputDescriptor = {
//         id: wasmDesc.id,
//         constraints,
//       };

//       if (wasmDesc.name !== undefined) inputDescriptor.name = wasmDesc.name;
//       if (wasmDesc.purpose !== undefined)
//         inputDescriptor.purpose = wasmDesc.purpose;

//       return inputDescriptor;
//     });

//   const presentationDefinition: PresentationDefinition = {
//     id: wasmPresentationDefinition.id,
//     input_descriptors: inputDescriptors,
//   };

//   if (wasmPresentationDefinition.name !== undefined)
//     presentationDefinition.name = wasmPresentationDefinition.name;

//   if (wasmPresentationDefinition.purpose !== undefined)
//     presentationDefinition.purpose = wasmPresentationDefinition.purpose;

//   if (wasmPresentationDefinition.submission_requirements !== undefined) {
//     presentationDefinition.submission_requirements =
//       wasmPresentationDefinition.submission_requirements.map((wasmReq) =>
//         submissionRequirementFromWASM(wasmReq)
//       );
//   }

//   return presentationDefinition;
// };

// const submissionRequirementFromWASM = (
//   wasmReq: wasm.WasmSubmissionRequirement
// ): SubmissionRequirement => {
//   const submissionRequirement: SubmissionRequirement = {
//     rule: { rule: wasmReq.rule.rule },
//   };

//   if (wasmReq.from !== undefined) submissionRequirement.from = wasmReq.from;
//   if (wasmReq.from_nested !== undefined) {
//     submissionRequirement.from_nested = wasmReq.from_nested.map((nestedReq) =>
//       submissionRequirementFromWASM(nestedReq)
//     );
//   }
//   if (wasmReq.name !== undefined) submissionRequirement.name = wasmReq.name;
//   if (wasmReq.purpose !== undefined)
//     submissionRequirement.purpose = wasmReq.purpose;
//   if (wasmReq.count !== undefined) submissionRequirement.count = wasmReq.count;
//   if (wasmReq.min !== undefined) submissionRequirement.min = wasmReq.min;
//   if (wasmReq.max !== undefined) submissionRequirement.max = wasmReq.max;

//   return submissionRequirement;
// };
