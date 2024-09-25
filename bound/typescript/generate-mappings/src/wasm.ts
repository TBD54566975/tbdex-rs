import * as ts from "typescript";

export type WasmClass = {
  className: string;
  members: WasmClassMember[];
  constructorParams: string[];
};

export type WasmClassMember = {
  wasmName: string; // always snake case
  tsName: string; // camel case except for PEX types
  type: string;
  isWasmClass: boolean;
  isNullable: boolean;
};

const toCamelCase = (snakeCase: string): string => {
  return snakeCase.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
};

const isNullable = (typeNode: ts.TypeNode): boolean => {
  const type = typeNode.getText();
  return type === "any" || type.includes("undefined") || type.includes("null");
};

const PEX_CONST_VALUE = "const_value";
const PEX_NAMES = [
  "input_descriptors",
  "submission_requirements",
  "from_nested",
  PEX_CONST_VALUE,
];

export const readWasmClasses = (code: string): WasmClass[] => {
  const classes: WasmClass[] = [];

  const sourceFile = ts.createSourceFile(
    "tmp.ts",
    code,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TS
  );

  ts.forEachChild(sourceFile, (node) => {
    if (ts.isClassDeclaration(node) && node.name) {
      if (!node.name.text.startsWith("Wasm")) {
        return;
      }

      const c: WasmClass = {
        className: node.name.text.slice(4),
        members: [],
        constructorParams: [],
      };

      node.members.forEach((member) => {
        if (ts.isPropertyDeclaration(member) && member.name && member.type) {
          const wasmName = member.name.getText();

          let tsName = toCamelCase(wasmName);
          // PEX names are the exception to the rule b/c they're snake case
          if (PEX_NAMES.includes(wasmName)) {
            if (wasmName === PEX_CONST_VALUE) tsName = "const";
            else tsName = wasmName;
          }

          let type = member.type.getText();
          if (type.includes(" | undefined"))
            type = type.replace(" | undefined", ""); // we use `?` syntax instead
          if (type.endsWith("[]") && type.startsWith("("))
            type = type.slice(1, -3) + "[]"; // ex. `(string)[]` slice out the `()`
          if (type.startsWith("Wasm")) type = type.slice(4);

          const classMember: WasmClassMember = {
            wasmName,
            tsName,
            type,
            isNullable: isNullable(member.type),
            isWasmClass:
              member.type.getText().startsWith("Wasm") ||
              member.type.getText().startsWith("(Wasm"),
          };

          c.members.push(classMember);
        }

        if (ts.isConstructorDeclaration(member)) {
          member.parameters.forEach((param) => {
            if (ts.isParameter(param) && param.name) {
              const wasmName = param.name.getText();
              let tsName = toCamelCase(wasmName);
              // PEX names are the exception to the rule b/c they're snake case
              if (PEX_NAMES.includes(wasmName)) {
                if (wasmName === PEX_CONST_VALUE) tsName = "const";
                else tsName = wasmName;
              }

              c.constructorParams.push(tsName);
            }
          });
        }
      });

      classes.push(c);
    }
  });

  return classes;
};
