import { BearerDid } from "../bearer-did";
import { Balance } from "../resources/balance";
import wasm from "../wasm";

export const getBalances = async (
  pfiDidUri: string,
  bearerDid: BearerDid
): Promise<Balance[]> => {
  const wasmBalances = wasm.get_balances(pfiDidUri, bearerDid.toWASM());
  return (await wasmBalances).map(Balance.fromWASM);
};
