package tbdex.sdk.rust

import java.io.File

object SystemArchitecture {
    @Volatile
    private var isSet = false

    fun set() {
        if (!isSet) {
            synchronized(this) {
                if (!isSet) {
                    val arch = System.getProperty("os.arch")?.lowercase() ?: throw Exception("Unable to get OS arch")
                    val name = System.getProperty("os.name")?.lowercase() ?: throw Exception("Unable to get OS name")

                    when {
                        name.contains("mac") && arch.contains("aarch64") ->
                            System.setProperty("uniffi.component.tbdex.libraryOverride", "tbdex_uniffi_aarch64_apple_darwin")

                        name.contains("mac") && arch.contains("x86_64") ->
                            System.setProperty("uniffi.component.tbdex.libraryOverride", "tbdex_uniffi_x86_64_apple_darwin")

                        name.contains("linux") && arch.contains("amd64") -> {
                            val osRelease = File("/etc/os-release")
                            if (osRelease.exists()) {
                                val osReleaseContent = osRelease.readText().lowercase()
                                when {
                                    osReleaseContent.contains("ubuntu") ->
                                        throw Exception("Ubuntu amd64 not yet supported")

                                    osReleaseContent.contains("alpine") ->
                                        throw Exception("Alpine amd64 not yet supported")

                                    else -> throw Exception("Unsupported OS arch $osReleaseContent")
                                }
                            } else {
                                throw Exception("Linux /etc/os-release not found")
                            }
                        }

                        else -> throw Exception("Unsupported OS arch $arch $name")
                    }
                    isSet = true
                }
            }
        }
    }
}
