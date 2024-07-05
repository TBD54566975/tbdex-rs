package tbdex.sdk.web5

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

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