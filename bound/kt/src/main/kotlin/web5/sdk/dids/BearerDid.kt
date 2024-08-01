package web5.sdk.dids

import tbdex.sdk.rust.SystemArchitecture
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.keys.KeyManager
import tbdex.sdk.rust.DidData as RustCoreDid
import tbdex.sdk.rust.DocumentData as RustCoreDocument
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid

typealias Did = RustCoreDid
typealias Document = RustCoreDocument

class BearerDid private constructor(
    val did: Did,
    val document: Document,
    val keyManager: KeyManager,
    internal val rustCoreBearerDid: RustCoreBearerDid
) {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromPortableDid(portableDid: PortableDid): BearerDid {
            val rustCoreBearerDid = RustCoreBearerDid.fromPortableDid(portableDid.rustCorePortableDid)
            val data = rustCoreBearerDid.getData()

            val keyManager = InMemoryKeyManager(portableDid.privateKeys)

            return BearerDid(data.did, data.document, keyManager, rustCoreBearerDid)
        }
    }
}