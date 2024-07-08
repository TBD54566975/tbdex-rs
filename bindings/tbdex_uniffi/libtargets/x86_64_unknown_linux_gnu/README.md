## Known system dependencies

> [!NOTE]
>
> If we decide to bundle all transitive dependencies into the shared library, then these are the set of dependencies for which we need to include.

```shell
root@237b102e4327:/usr/src/myapp# ldd target/release/libtbdex_uniffi.so
	libssl.so.3 => /lib/x86_64-linux-gnu/libssl.so.3 (0x00007ffffee3c000)
	libcrypto.so.3 => /lib/x86_64-linux-gnu/libcrypto.so.3 (0x00007ffffe9f8000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007ffffe9d8000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007ffffe8f1000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007ffffe6c8000)
	/lib64/ld-linux-x86-64.so.2 (0x00007ffffffc4000)
```