import * as ts from "typescript";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const tsCode = fs.readFileSync(
  path.resolve(__dirname, "../../src/wasm/generated.d.ts"),
  "utf8"
);

const sourceFile = ts.createSourceFile(
  "tmp.ts",
  tsCode,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TS
);

type WasmType = {
  name: string;
  properties: Array<{
    camelCase: string;
    snakeCase: string;
    type: string;
    isNullable: boolean;
    isWasmType: boolean;
  }>;
  constructorParams: Array<string>;
};

function toCamelCase(snakeCase: string): string {
  return snakeCase.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

function isNullable(typeNode: ts.TypeNode): boolean {
  return (
    typeNode.getText().includes("undefined") ||
    typeNode.getText().includes("null")
  );
}

const pex_snake_case = [
  "input_descriptors",
  "submission_requirements",
  "from_nested",
  "const_value",
];

const result: WasmType[] = [];

ts.forEachChild(sourceFile, (node) => {
  if (ts.isClassDeclaration(node) && node.name) {
    const className = node.name.text.startsWith("Wasm")
      ? node.name.text.slice(4)
      : node.name.text;
    const properties: Array<{
      snakeCase: string;
      camelCase: string;
      type: string;
      isNullable: boolean;
      isWasmType: boolean;
    }> = [];
    const constructorParams: Array<string> = [];

    node.members.forEach((member) => {
      if (ts.isPropertyDeclaration(member) && member.name && member.type) {
        const snakeCase = member.name.getText();
        const camelCase = pex_snake_case.find((x) => x === snakeCase)
          ? snakeCase === "const_value"
            ? "const"
            : snakeCase
          : toCamelCase(snakeCase);
        const nullable = isNullable(member.type);

        let propType = member.type.getText().replace(" | undefined", "");
        propType =
          propType.endsWith("[]") && propType.startsWith("(")
            ? propType.slice(1, -3) + "[]"
            : propType;
        let isWasmType = false;
        if (propType.startsWith("Wasm")) {
          isWasmType = true;
          propType = propType.slice(4);
        }

        properties.push({
          snakeCase,
          camelCase,
          type: propType,
          isNullable: nullable,
          isWasmType,
        });
      }

      if (ts.isConstructorDeclaration(member)) {
        member.parameters.forEach((param) => {
          if (ts.isParameter(param) && param.name) {
            const snakeCase = param.name.getText();
            const camelCase = pex_snake_case.find((x) => x === snakeCase)
              ? snakeCase === "const_value"
                ? "const"
                : snakeCase
              : toCamelCase(snakeCase);

            constructorParams.push(camelCase);
          }
        });
      }
    });

    result.push({
      name: className,
      properties,
      constructorParams,
    });
  }
});

let code = 'import wasm from "./";\n\n';
result.forEach((t) => {
  code += `
    export type ${t.name} = {
      ${t.properties
        .map((p) => `\t${p.camelCase}${p.isNullable ? "?" : ""}: ${p.type};\n`)
        .join("")}
    };
    
    export namespace ${t.name} {
      export const toWASM = (
        obj: ${t.name}
      ): wasm.Wasm${t.name} => {
        return new wasm.Wasm${t.name}(
          ${t.constructorParams
            .map((p) => {
              const property = t.properties.find((x) => x.camelCase === p);
              if (property?.isWasmType) {
                if (property.type.endsWith("[]")) {
                  return `obj.${p}?.map(${property.type.slice(0, -2)}.toWASM)`;
                } else if (property.isNullable) {
                  return `obj.${p} ? ${property.type}.toWASM(obj.${p}) : undefined`;
                }

                return `${property.type}.toWASM(obj.${p})`;
              }

              return `obj.${p}`;
            })
            .join(",\n\t")}
        );
      };

      export const fromWASM = (
        obj: wasm.Wasm${t.name}
      ): ${t.name} => {
        const result: ${t.name} = {
          \t${t.properties
            .filter((x) => !x.isNullable)
            .map((p) => {
              if (p.isWasmType) {
                if (p.type.endsWith("[]")) {
                  return `${p.camelCase}: obj.${
                    p.snakeCase
                  }?.map(${p.type.slice(0, -2)}.fromWASM)`;
                }

                return `${p.camelCase}: ${p.type}.fromWASM(obj.${p.snakeCase}),\n`;
              }

              return `${p.camelCase}: obj.${p.snakeCase},\n`;
            })
            .join("")}
        };

        ${t.properties
          .filter((x) => x.isNullable)
          .map(
            (p) => `
              if (obj.${p.snakeCase} !== undefined) 
                result.${p.camelCase} = ${
              p.isWasmType
                ? p.type.endsWith("[]")
                  ? `obj.${p.snakeCase}?.map(${p.type.slice(0, -2)}.fromWASM)`
                  : `${p.type}.fromWASM(obj.${p.snakeCase})`
                : `obj.${p.snakeCase}`
            };
            `
          )
          .join("")}

        return result;
      };
    }`;
});

fs.writeFileSync(path.resolve(__dirname, "../../src/wasm/mappings.ts"), code);
