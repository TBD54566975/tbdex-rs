package tbdex.sdk.web5

import tbdex.sdk.rust.JwkData as RustCoreJwk
import tbdex.sdk.rust.KeyManager as RustCoreKeyManager

typealias Jwk = RustCoreJwk

interface KeyManager {
    fun getSigner(publicJwk: Jwk): Signer
    fun getRustCoreKeyManager(): RustCoreKeyManager
}