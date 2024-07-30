package tbdex.sdk.web5

import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid as Web5BearerDid

internal class InternalBearerDid private constructor(
    internal val rustCoreBearerDid: RustCoreBearerDid,
) {
    companion object {
        fun fromWeb5(web5BearerDid: Web5BearerDid): InternalBearerDid {
            return InternalBearerDid(
                RustCoreBearerDid(
                    uri = web5BearerDid.did.uri,
                    keyManager = InternalKeyManager.fromWeb5(web5BearerDid.keyManager)
                )
            )
        }
    }
}