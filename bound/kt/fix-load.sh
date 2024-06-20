#!/bin/bash

# Path to your Kotlin file
FILE="src/main/kotlin/tbdex/sdk/rust/tbdex.kt"

# Add the import line after the last import
awk '/^import/ {print; found=1; next} found && !/^import/ {print "import java.nio.file.Files"; found=0} {print}' $FILE > tmpfile && mv tmpfile $FILE

# Replace the specified block of code using a more BSD-friendly approach
sed -i '' '/loadIndirect<UniffiLib>(componentName = "tbdex")/,/also { lib: UniffiLib ->/c\
            val tempDir = Files.createTempDirectory("library")\
            val libraryPath = tempDir.resolve("libtbdex_uniffi.dylib")\
            Thread.currentThread().contextClassLoader.getResourceAsStream("natives/libtbdex_uniffi.dylib").use { input ->\
                Files.copy(input, libraryPath)\
            }\
            libraryPath.toFile().deleteOnExit()\
            val lib = Native.load(libraryPath.toString(), UniffiLib::class.java)\
            lib.also {' $FILE
