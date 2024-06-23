package tbdex.sdk.web5

import tbdex.sdk.rust.Signer as RustCoreSigner

interface Signer {
    fun sign(payload: ByteArray): ByteArray
}

class OuterSigner : Signer {
    private val rustCoreSigner: RustCoreSigner

    constructor(rustCoreSigner: RustCoreSigner) {
        this.rustCoreSigner = rustCoreSigner
    }

    @OptIn(ExperimentalUnsignedTypes::class)
    override fun sign(payload: ByteArray): ByteArray {
        return this.rustCoreSigner.sign(payload.toUByteArray().toList())
    }
}