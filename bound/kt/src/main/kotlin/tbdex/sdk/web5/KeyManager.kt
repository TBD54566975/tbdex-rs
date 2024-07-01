package tbdex.sdk.web5

import tbdex.sdk.rust.JwkData as RustCoreJwk
import tbdex.sdk.rust.KeyManager as RustCoreKeyManager

typealias Jwk = RustCoreJwk

interface KeyManager {
    fun getSigner(publicJwk: Jwk): Signer
    fun getRustCoreKeyManager(): RustCoreKeyManager
}

internal class ToOuterKeyManager(private val innerKeyManager: RustCoreKeyManager): KeyManager {
    override fun getSigner(publicJwk: Jwk): Signer {
        return ToOuterSigner(innerKeyManager.getSigner(publicJwk))
    }

    override fun getRustCoreKeyManager(): RustCoreKeyManager {
        return this.innerKeyManager
    }
}