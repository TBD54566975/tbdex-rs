package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.signers.ToOuterSigner
import tbdex.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager
import tbdex.sdk.rust.KeyManager as RustCoreKeyManager

internal class InMemoryKeyManager : KeyManager {
    private val rustCoreInMemoryKeyManager = RustCoreInMemoryKeyManager()

    constructor(privateJwks: List<Jwk>) {
        privateJwks.forEach {
            this.rustCoreInMemoryKeyManager.importPrivateJwk(it)
        }
    }

    override fun getSigner(publicJwk: Jwk): Signer {
        val innerSigner = this.rustCoreInMemoryKeyManager.getSigner(publicJwk)
        return ToOuterSigner(innerSigner)
    }

    override fun getRustCoreKeyManager(): RustCoreKeyManager {
        return this.rustCoreInMemoryKeyManager.getAsKeyManager()
    }

    fun importPrivateJwk(privateJwk: Jwk): Jwk {
        return this.rustCoreInMemoryKeyManager.importPrivateJwk(privateJwk)
    }
}