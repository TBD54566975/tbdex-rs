package tbdex.sdk.web5

import tbdex.sdk.rust.Signer
import web5.sdk.crypto.signers.Signer as Web5Signer

internal class InnerSigner private constructor(
    private val web5Signer: Web5Signer
): Signer {
    companion object {
        fun fromWeb5(web5Signer: Web5Signer): InnerSigner {
            return InnerSigner(web5Signer)
        }
    }

    override fun sign(payload: ByteArray): ByteArray {
        return this.web5Signer.sign(payload)
    }
}