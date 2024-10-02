import { Offering } from "../resources/offering";
import wasm from "../wasm";

export const getOfferings = async (pfiDidUri: string): Promise<Offering[]> => {
  const offeringsJsonString = await wasm.get_offerings(pfiDidUri);
  const offeringsJsonArray = JSON.parse(offeringsJsonString);
  const offerings: Offering[] = [];
  for (const offeringObject of offeringsJsonArray) {
    offerings.push(
      new Offering(
        offeringObject.metadata,
        offeringObject.data,
        offeringObject.signature
      )
    );
  }
  return offerings;
};
