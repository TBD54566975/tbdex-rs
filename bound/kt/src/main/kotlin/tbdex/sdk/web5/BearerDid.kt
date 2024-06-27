package tbdex.sdk.web5

import tbdex.sdk.rust.DidData as RustCoreDid
import tbdex.sdk.rust.DocumentData as RustCoreDocument
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid

typealias Did = RustCoreDid
typealias Document = RustCoreDocument

class BearerDid {
    val did: Did
    val document: Document
    val keyManager: KeyManager

    val rustCoreBearerDid: RustCoreBearerDid

    constructor(uri: String, keyManager: KeyManager) {
        this.rustCoreBearerDid = RustCoreBearerDid(uri, keyManager.getRustCoreKeyManager())

        this.did = this.rustCoreBearerDid.getData().did
        this.document = this.rustCoreBearerDid.getData().document
        this.keyManager = keyManager
    }

    fun getSigner(): Signer {
        // TODO currently hardcoding to first VM
        val keyId = this.document.verificationMethod.first().id
        val innerSigner = this.rustCoreBearerDid.getSigner(keyId)
        return OuterSigner(innerSigner)
    }
}