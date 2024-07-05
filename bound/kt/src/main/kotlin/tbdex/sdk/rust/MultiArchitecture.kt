package tbdex.sdk.rust

import java.io.File

private enum class SystemArchitecture {
    MAC_APPLE_SILICON,
    MAC_INTEL,
    LINUX_UBUNTU_AMD64,
    LINUX_ALPINE_AMD64,
    UNSUPPORTED
}

private fun getSystemArchitecture(): SystemArchitecture {
    val arch = System.getProperty("os.arch")?.lowercase() ?: return SystemArchitecture.UNSUPPORTED
    val name = System.getProperty("os.name")?.lowercase() ?: return SystemArchitecture.UNSUPPORTED

    return when {
        name.contains("mac") && arch.contains("aarch64") -> SystemArchitecture.MAC_APPLE_SILICON
        name.contains("mac") && arch.contains("x86_64") -> SystemArchitecture.MAC_INTEL

        name.contains("linux") && arch.contains("amd64") -> {
            val osRelease = File("/etc/os-release")
            if (osRelease.exists()) {
                val osReleaseContent = osRelease.readText().lowercase()
                when {
                    osReleaseContent.contains("ubuntu") -> SystemArchitecture.LINUX_UBUNTU_AMD64
                    osReleaseContent.contains("alpine") -> SystemArchitecture.LINUX_ALPINE_AMD64
                    else -> SystemArchitecture.UNSUPPORTED
                }
            } else {
                SystemArchitecture.UNSUPPORTED
            }
        }

        else -> SystemArchitecture.UNSUPPORTED
    }
}

class MultiArchitecture {
    companion object {
        init {
            val systemArchitecture = getSystemArchitecture()
            when (systemArchitecture) {
                SystemArchitecture.MAC_APPLE_SILICON -> {
                    System.setProperty("uniffi.component.tbdex.libraryOverride", "tbdex_uniffi_aarch64_apple_darwin")
                }
                SystemArchitecture.MAC_INTEL -> {
                    System.setProperty("uniffi.component.tbdex.libraryOverride", "tbdex_uniffi_x86_64_apple_darwin")
                }
                SystemArchitecture.LINUX_UBUNTU_AMD64 -> {
                    throw Exception("${SystemArchitecture.LINUX_UBUNTU_AMD64} not yet supported")
                }
                SystemArchitecture.LINUX_ALPINE_AMD64 -> {
                    throw Exception("${SystemArchitecture.LINUX_ALPINE_AMD64} not yet supported")
                }
                else -> {
                    throw Exception("unsupported sysytem architecture")
                }
            }
        }

        fun getOfferings(pfiDidUri: String): List<tbdex.sdk.rust.Offering> {
            return tbdex.sdk.rust.getOfferings(pfiDidUri)
        }
    }
}