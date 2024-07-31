package web5.sdk.crypto.signers

import tbdex.sdk.rust.Signer as RustCoreSigner

interface Signer {
    fun sign(payload: ByteArray): ByteArray
}

internal class ToOuterSigner : Signer {
    private val rustCoreSigner: RustCoreSigner

    constructor(rustCoreSigner: RustCoreSigner) {
        this.rustCoreSigner = rustCoreSigner
    }

    override fun sign(payload: ByteArray): ByteArray {
        return this.rustCoreSigner.sign(payload)
    }
}