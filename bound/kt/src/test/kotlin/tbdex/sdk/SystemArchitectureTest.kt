package tbdex.sdk

import org.junit.jupiter.api.Test
import tbdex.sdk.rust.UniffiLib

class SystemArchitectureTest {
    @Test
    fun `can load shared library`() {
        System.setProperty("TBDEX_SDK_LOG_LEVEL", "debug")
        UniffiLib.INSTANCE
        println("Successfully loaded shared library for ${System.getProperty("uniffi.component.tbdex.libraryOverride")}")
    }
}