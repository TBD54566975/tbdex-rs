import tbdex.sdk.rust.UniffiLib;

/**
 * A simple main class to act as an acceptance smoke test for the Kotlin
 * tbDEX binary.
 *
 * See README.md in this folder for usage and purpose.
 */
public class TbdexAcceptanceTest {

    public static void main(String... args) {
        System.setProperty("TBDEX_SDK_LOG_LEVEL", "debug");
        UniffiLib.Companion.getINSTANCE$tbdex();
        System.out.println(
                "Successfully loaded shared library for " +
                        System.getProperty("uniffi.component.tbdex.libraryOverride"));
    }
}
