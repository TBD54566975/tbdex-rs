## Known system dependencies

> [!NOTE]
>
> To build as a dynamic library, disable static linking.
> 
> `RUSTFLAGS="-C target-feature=-crt-s
tatic" cargo build --release --package tbdex_uniffi`

> [!NOTE]
>
> If we decide to dynamically install system dependencies using a maven feature, then these are the list of dependencies we will need to ensure are installed.

```shell
/usr/src/myapp # ldd target/release/libtbdex_uniffi.so
	/lib/ld-musl-x86_64.so.1 (0x7ffffff5d000)
	libssl.so.3 => /lib/libssl.so.3 (0x7ffffedbd000)
	libcrypto.so.3 => /lib/libcrypto.so.3 (0x7ffffe975000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x7ffffe951000)
	libc.musl-x86_64.so.1 => /lib/ld-musl-x86_64.so.1 (0x7ffffff5d000)
```