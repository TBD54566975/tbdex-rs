package tbdex.sdk.web5

import tbdex.sdk.rust.Signer
import web5.sdk.crypto.signers.Signer as Web5Signer

internal class InternalSigner private constructor(
    private val web5Signer: Web5Signer
): Signer {
    companion object {
        fun fromWeb5(web5Signer: Web5Signer): InternalSigner {
            return InternalSigner(web5Signer)
        }
    }

    override fun sign(payload: ByteArray): ByteArray {
        return this.web5Signer.sign(payload)
    }
}