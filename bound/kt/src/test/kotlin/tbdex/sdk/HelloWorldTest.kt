package tbdex.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import tbdex.sdk.rust.helloWorld

class HelloWorldTests {
    @Test
    fun `can helloWorld`() {
        helloWorld()
    }
}