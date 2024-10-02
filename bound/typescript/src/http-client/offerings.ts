import { Offering } from "../resources/offering";
import wasm from "../wasm";

export const getOfferings = async (pfiDidUri: string): Promise<Offering[]> => {
  const wasmOfferings = await wasm.get_offerings(pfiDidUri);
  return wasmOfferings.map(Offering.fromWASM);
};
