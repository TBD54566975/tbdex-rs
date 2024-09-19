package tbdex.sdk.resources

import tbdex.sdk.rust.ResourceKind as RustCoreResourceKind
import tbdex.sdk.rust.ResourceMetadataData as RustCoreResourceMetadata

/**
 * Represents the different kinds of resources in the tbDEX protocol.
 *
 * A resource can be either an OFFERING or a BALANCE.
 */
enum class ResourceKind {
    OFFERING,   // Represents an offering resource in the tbDEX protocol
    BALANCE;    // Represents a balance resource in the tbDEX protocol

    companion object {
        /**
         * Converts a RustCore resource kind into a Kotlin `ResourceKind`.
         *
         * @param rustCore The RustCore representation of the resource kind.
         * @return The Kotlin `ResourceKind`.
         */
        internal fun fromRustCore(rustCore: RustCoreResourceKind): ResourceKind {
            return when (rustCore) {
                RustCoreResourceKind.BALANCE -> BALANCE
                RustCoreResourceKind.OFFERING -> OFFERING
            }
        }
    }
}

/**
 * Represents metadata associated with a tbDEX resource.
 *
 * Metadata provides information about the resource, such as its type, origin, creation time, and protocol version.
 *
 * @property kind The kind of resource (e.g., OFFERING, BALANCE).
 * @property from The DID of the sender of the resource.
 * @property id The unique identifier for the resource.
 * @property protocol The version of the tbDEX protocol in use.
 * @property createdAt The timestamp when the resource was created (in ISO 8601 format).
 * @property updatedAt The optional timestamp of the last update to the resource (in ISO 8601 format).
 */
data class ResourceMetadata (
    val kind: ResourceKind,
    val from: String,
    val id: String,
    val protocol: String,
    val createdAt: String,
    val updatedAt: String?
) {
    companion object {
        /**
         * Converts a RustCore resource metadata into a Kotlin `ResourceMetadata`.
         *
         * @param rustCore The RustCore representation of the resource metadata.
         * @return The Kotlin `ResourceMetadata`.
         */
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
