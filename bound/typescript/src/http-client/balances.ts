import { BearerDid } from "../bearer-did";
import { Balance } from "../resources/balance";
import wasm from "../wasm";

export const getBalances = async (
  pfiDidUri: string,
  bearerDid: BearerDid
): Promise<Balance[]> => {
  const json = await wasm.get_balances(pfiDidUri, bearerDid.toWASM());
  const balances = JSON.parse(json);
  return balances.map(
    (x: Balance) => new Balance(x.metadata, x.data, x.signature)
  );
};
