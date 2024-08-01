package web5.sdk.crypto.keys

import tbdex.sdk.rust.SystemArchitecture
import tbdex.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

internal class InMemoryKeyManager private constructor(
    internal val rustCoreInMemoryKeyManager: RustCoreInMemoryKeyManager,
): KeyManager {
    init {
        SystemArchitecture.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        fun fromPrivateJwks(privateJwks: List<Jwk>): InMemoryKeyManager {
            val rustCoreInMemoryKeyManager = RustCoreInMemoryKeyManager()
            privateJwks.forEach {
                rustCoreInMemoryKeyManager.importPrivateJwk(it)
            }

            return InMemoryKeyManager(rustCoreInMemoryKeyManager)
        }
    }
}