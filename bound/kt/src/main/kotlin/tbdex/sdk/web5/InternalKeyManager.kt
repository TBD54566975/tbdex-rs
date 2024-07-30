package tbdex.sdk.web5

import tbdex.sdk.rust.JwkData
import tbdex.sdk.rust.Signer
import web5.sdk.crypto.keys.KeyManager as Web5KeyManager
import web5.sdk.crypto.keys.Jwk as Web5Jwk

internal class InternalKeyManager private constructor(
    private val web5KeyManager: Web5KeyManager
): tbdex.sdk.rust.KeyManager {
    companion object {
        fun fromWeb5(keyManager: Web5KeyManager): InternalKeyManager {
            return InternalKeyManager(keyManager)
        }
    }

    override fun getSigner(publicJwk: JwkData): Signer {
        val web5PublicJwk = Web5Jwk(
            publicJwk.alg,
            publicJwk.kty,
            publicJwk.crv,
            publicJwk.d,
            publicJwk.x,
            publicJwk.y
        )

        val web5Signer = this.web5KeyManager.getSigner(web5PublicJwk)

        return InternalSigner.fromWeb5(web5Signer)
    }
}