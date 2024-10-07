import { BearerDid } from "../dids/bearer-did";
import { Balance } from "../resources/balance";
import wasm from "../wasm";

export const getBalances = async (
  pfiDidUri: string,
  bearerDid: BearerDid
): Promise<Balance[]> => {
  const json = await wasm.get_balances(pfiDidUri, bearerDid.toWASM());
  const arr = JSON.parse(json);
  return arr.map((x: Balance) => new Balance(x.metadata, x.data, x.signature));
};
