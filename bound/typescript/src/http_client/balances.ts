import { BearerDid } from "../bearer-did";
import { Balance } from "../resources/balance";
import wasm from "../wasm";

export const getBalances = async (
  pfiDidUri: string,
  bearerDid: BearerDid
): Promise<Balance[]> => {
  const wasmBalances = await wasm.get_balances(pfiDidUri, bearerDid.toWASM());
  return wasmBalances.map(Balance.fromWASM);
};
