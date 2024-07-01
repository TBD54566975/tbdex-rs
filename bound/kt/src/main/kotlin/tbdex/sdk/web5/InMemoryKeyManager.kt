package tbdex.sdk.web5

import tbdex.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager
import tbdex.sdk.rust.KeyManager as RustCoreKeyManager

class InMemoryKeyManager : KeyManager {
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