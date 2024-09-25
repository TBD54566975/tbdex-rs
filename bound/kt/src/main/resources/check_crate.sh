#!/bin/bash

files=(
    "libtbdex_uniffi_aarch64_apple_darwin.dylib"
    "libtbdex_uniffi_x86_64_apple_darwin.dylib"
    "libweb5_uniffi_x86_64_unknown_linux_gnu.so"
    "libweb5_uniffi_x86_64_unknown_linux_musl.so"
)

crate_name="crate_name" # Replace with the actual crate name

for file in "${files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "Checking $file for $crate_name..."
        nm -D "$file" | grep "$crate_name" && echo "$crate_name found in $file with gnm"
        objdump -T "$file" | grep "$crate_name" && echo "$crate_name found in $file with gobjdump"
    else
        echo "$file does not exist"
    fi
done

