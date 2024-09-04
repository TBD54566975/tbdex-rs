package tbdex.sdk.rust

import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.signers.Signer
import web5.sdk.dids.*
import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import tbdex.sdk.rust.DidData as RustCoreDid
import tbdex.sdk.rust.DocumentData as RustCoreDocument
import tbdex.sdk.rust.KeyManager as RustCoreKeyManager
import tbdex.sdk.rust.VerificationMethodData as RustCoreVerificationMethod
import tbdex.sdk.rust.JwkData as RustCoreJwk
import tbdex.sdk.rust.ServiceData as RustCoreService
import tbdex.sdk.rust.Signer as RustCoreSigner

internal fun RustCoreDid.Companion.fromWeb5(did: Did): RustCoreDid {
    return RustCoreDid(
        did.uri,
        did.url,
        did.method,
        did.id,
        did.params,
        did.path,
        did.query,
        did.fragment
    )
}

internal fun RustCoreDocument.Companion.fromWeb5(document: Document): RustCoreDocument {
    return RustCoreDocument(
        document.id,
        document.context,
        document.controller,
        document.alsoKnownAs,
        document.verificationMethod.map { RustCoreVerificationMethod.fromWeb5(it) },
        document.authentication,
        document.assertionMethod,
        document.keyAgreement,
        document.capabilityInvocation,
        document.capabilityDelegation,
        document.service?.map { RustCoreService.fromWeb5(it) },
    )
}

internal fun RustCoreVerificationMethod.Companion.fromWeb5(verificationMethod: VerificationMethod): RustCoreVerificationMethod {
    return RustCoreVerificationMethod(
        verificationMethod.id,
        verificationMethod.type,
        verificationMethod.controller,
        RustCoreJwk.fromWeb5(verificationMethod.publicKeyJwk)
    )
}

internal fun RustCoreJwk.Companion.fromWeb5(jwk: Jwk): RustCoreJwk {
    return RustCoreJwk(
        jwk.alg,
        jwk.kty,
        jwk.crv,
        jwk.d,
        jwk.x,
        jwk.y
    )
}

internal fun RustCoreService.Companion.fromWeb5(service: Service): RustCoreService {
    return RustCoreService(
        service.id,
        service.type,
        service.serviceEndpoint
    )
}

internal fun RustCoreJwk.toWeb5(): Jwk {
    return Jwk(
        this.alg,
        this.kty,
        this.crv,
        this.x,
        this.y,
        this.d,
    )
}

internal class ToInnerSigner(private val signer: Signer) : RustCoreSigner {
    override fun sign(payload: ByteArray): ByteArray {
        return signer.sign(payload)
    }
}

internal class ToInnerKeyManager(private val keyManager: KeyManager) : RustCoreKeyManager {
    override fun importPrivateJwk(privateJwk: RustCoreJwk): RustCoreJwk {
        val jwk = keyManager.importPrivateJwk(privateJwk.toWeb5())
        return RustCoreJwk.fromWeb5(jwk)
    }

    override fun getSigner(publicJwk: RustCoreJwk): RustCoreSigner {
        val signer = keyManager.getSigner(publicJwk.toWeb5())
        val innerSigner = ToInnerSigner(signer)
        return innerSigner
    }
}

internal fun RustCoreBearerDid.Companion.fromWeb5(bearerDid: BearerDid): RustCoreBearerDid {
    val did = RustCoreDid.fromWeb5(bearerDid.did)
    val document = RustCoreDocument.fromWeb5(bearerDid.document)
    val keyManager = ToInnerKeyManager(bearerDid.keyManager)

    return RustCoreBearerDid(did, document, keyManager)
}