package tbdex.sdk.web5

import tbdex.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.dids.BearerDid as Web5BearerDid

internal class InnerBearerDid private constructor(
    internal val rustCoreBearerDid: RustCoreBearerDid,
) {
    companion object {
        fun fromWeb5(web5BearerDid: Web5BearerDid): InnerBearerDid {
            return InnerBearerDid(
                RustCoreBearerDid(
                    uri = web5BearerDid.did.uri,
                    keyManager = InnerKeyManager.fromWeb5(web5BearerDid.keyManager)
                )
            )
        }
    }
}