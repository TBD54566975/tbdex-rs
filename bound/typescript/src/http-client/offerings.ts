import { Offering } from "../resources/offering";
import wasm from "../wasm";

export const getOfferings = async (pfiDidUri: string): Promise<Offering[]> => {
  const offerings_json_string = await wasm.get_offerings(pfiDidUri);
  const offerings_json_array = JSON.parse(offerings_json_string);
  const offerings: Offering[] = [];
  for (const offering_obj of offerings_json_array) {
    offerings.push(
      new Offering(
        offering_obj.metadata,
        offering_obj.data,
        offering_obj.signature
      )
    );
  }
  return offerings;
};
