import wasm from "../wasm";

export type Did = {
  fragment?: string;
  id: string;
  method: string;
  params?: any;
  path?: string;
  query?: string;
  uri: string;
  url: string;
};

export namespace Did {
  export const parse = (uri: string): Did => {
    const did_json = wasm.parse_did(uri);
    const did: Did = JSON.parse(did_json);
    return did;
  };
}
