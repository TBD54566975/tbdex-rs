import { WasmClass } from "./wasm.js";

const EXCLUDE = [
  "KeyManager",
  "BearerDid",
  "Signer",
  "PortableDid",

  // resources
  "Offering",
  "Balance",

  // messages
  "Rfq",
  "Quote",
  "Order",
  "OrderInstructions",
  "Cancel",
  "OrderStatus",
  "Close",

  // http
  "GetExchangeResponseBody",
  "GetExchangesResponseBody",
  "CreateExchangeRequestBody",
  "UpdateExchangeRequestBody",
  "ReplyToRequestBody",
  "GetOfferingsResponseBody",
  "GetBalancesResponseBody",
  "Exchange"
];

export const generateToWASM = (wasmClass: WasmClass): string => `
  export const toWASM = (
    obj: ${wasmClass.className}
  ): wasm.Wasm${wasmClass.className} => {
    return new wasm.Wasm${wasmClass.className}(
      ${wasmClass.constructorParams
        .map((p) => {
          const member = wasmClass.members.find((x) => x.tsName === p);
          if (member?.isWasmClass) {
            if (member.type.endsWith("[]")) {
              // if array, then map over the property and call the toWASM() function for each
              return `obj.${p}?.map(${member.type.slice(0, -2)}.toWASM)`;
            } else if (member.isNullable) {
              // if nullable, then only call the toWASM() function if it's value is set
              return `obj.${p} ? ${member.type}.toWASM(obj.${p}) : undefined`;
            } else {
              // call the toWASM() function
              return `${member.type}.toWASM(obj.${p})`;
            }
          } else if (member?.type === "bigint") {
            return `BigInt(obj.${p})`;
          } else {
            return `obj.${p}`;
          }
        })
        .join(",")}
    )
  }
`;

export const generateFromWASM = (wasmClass: WasmClass): string => `
  export const fromWASM = (
    obj: wasm.Wasm${wasmClass.className}
  ): ${wasmClass.className} => {
    const result: ${wasmClass.className} = {
      ${wasmClass.members
        .filter((x) => !x.isNullable)
        .map((member) => {
          if (member.isWasmClass) {
            if (member.type.endsWith("[]")) {
              // if Wasm* array, then map over each and fromWASM()
              return `${member.tsName}: obj.${
                member.wasmName
              }?.map(${member.type.slice(0, -2)}.fromWASM)`;
            } else {
              // else if Wasm*, then call fromWASM()
              return `${member.tsName}: ${member.type}.fromWASM(obj.${member.wasmName}),`;
            }
          } else if (member.type === "bigint") {
            // special case for bigint
            return `${member.tsName}: Number(obj.${member.wasmName}),`;
          } else {
            // else just simple assignment
            return `${member.tsName}: obj.${member.wasmName},`;
          }
        })
        .join("")}
    };

    ${wasmClass.members
      .filter((x) => x.isNullable)
      .map((member) => {
        let code = `
            if (obj.${member.wasmName} !== undefined) 
              result.${member.tsName} = `;

        if (member.isWasmClass) {
          if (member.type.endsWith("[]"))
            code += `obj.${member.wasmName}?.map(${member.type.slice(
              0,
              -2
            )}.fromWASM)`;
          else code += `${member.type}.fromWASM(obj.${member.wasmName})`;
        } else {
          code += `obj.${member.wasmName}`;
        }

        return code;
      })
      .join("")}

    return result
  };
`;

export const generateMappingsCode = (wasmClasses: WasmClass[]): string => `
  import wasm from "./"

  ${wasmClasses
    .map((wasmClass) => {
      // todo
      if (EXCLUDE.includes(wasmClass.className)) return;

      return `
        export type ${wasmClass.className} = {
          ${wasmClass.members
            .map((m) => {
              if (m.type === "bigint")
                return `${m.tsName}${m.isNullable ? "?" : ""}: number`;
              else return `${m.tsName}${m.isNullable ? "?" : ""}: ${m.type}`;
            })
            .join("\n")}
        }

        export namespace ${wasmClass.className} {
          ${generateToWASM(wasmClass)}
          ${generateFromWASM(wasmClass)}
        }
      `;
    })
    .join("\n")}
`;
