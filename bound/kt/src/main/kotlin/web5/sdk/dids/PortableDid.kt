package web5.sdk.dids

import tbdex.sdk.rust.SystemArchitecture
import web5.sdk.crypto.keys.Jwk
import tbdex.sdk.rust.PortableDid as RustCorePortableDid

internal class PortableDid private constructor(
    val didUri: String,
    val document: Document,
    val privateKeys: List<Jwk>,
    internal val rustCorePortableDid: RustCorePortableDid
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromJsonString(json: String): PortableDid {
            val rustCorePortableDid = RustCorePortableDid(json)
            val data = rustCorePortableDid.getData()

            return PortableDid(data.didUri, data.document, data.privateJwks, rustCorePortableDid)
        }
    }
}