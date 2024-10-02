import { Offering } from "../resources/offering";
import wasm from "../wasm";

export const getOfferings = async (pfiDidUri: string): Promise<Offering[]> => {
  const json = await wasm.get_offerings(pfiDidUri);
  const arr = JSON.parse(json);
  return arr.map(
    (x: Offering) => new Offering(x.metadata, x.data, x.signature)
  );
};
