package tbdex.sdk

class TbdexException(
    val variant: String,
    override val message: String
) : Exception(message) {
    companion object {
        internal fun fromRustCore(e: tbdex.sdk.rust.TbdexException.Exception): TbdexException {
            return TbdexException(e.variant, e.msg)
        }
    }
}