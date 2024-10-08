import { Balance } from "./balance";
import { Offering } from "./offering";

export * from "./balance"
export * from "./offering"

export type Resource = Balance | Offering;

export type ResourceMetadata = {
  createdAt: string;
  from: string;
  id: string;
  kind: ResourceKind;
  protocol: string;
  updatedAt?: string;
};

export type ResourceKind = "offering" | "balance";
