package tbdex.sdk.resources

import tbdex.sdk.rust.ResourceKind as RustCoreResourceKind
import tbdex.sdk.rust.ResourceMetadataData as RustCoreResourceMetadata

enum class ResourceKind {
    OFFERING,
    BALANCE;

    companion object {
        internal fun fromRustCore(rustCore: RustCoreResourceKind): ResourceKind {
            return when (rustCore) {
                RustCoreResourceKind.BALANCE -> BALANCE
                RustCoreResourceKind.OFFERING -> OFFERING
            }
        }
    }
}

data class ResourceMetadata (
    val kind: ResourceKind,
    val from: String,
    val id: String,
    val protocol: String,
    val createdAt: String,
    val updatedAt: String?
) {
    companion object {
        internal fun fromRustCore(rustCore: RustCoreResourceMetadata): ResourceMetadata {
            return ResourceMetadata(
                ResourceKind.fromRustCore(rustCore.kind),
                rustCore.from,
                rustCore.id,
                rustCore.protocol,
                rustCore.createdAt,
                rustCore.updatedAt
            )
        }
    }
}