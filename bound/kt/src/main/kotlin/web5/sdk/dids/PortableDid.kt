package web5.sdk.dids

import tbdex.sdk.rust.SystemArchitecture
import web5.sdk.crypto.keys.Jwk
import tbdex.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid {
    val didUri: String
    val document: Document
    val privateKeys: List<Jwk>

    internal val rustCorePortableDid: RustCorePortableDid

    constructor(json: String) {
        this.rustCorePortableDid = RustCorePortableDid(json)

        this.didUri = rustCorePortableDid.getData().didUri
        this.document = rustCorePortableDid.getData().document
        this.privateKeys = rustCorePortableDid.getData().privateJwks
    }
}