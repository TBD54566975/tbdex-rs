# tbDEX Kotlin Acceptance Test

This is a simple Java main class which acts as an acceptance smoke test
for the tbDEX Kotlin binary distribution. The Kotlin unit tests execute in
Maven using the `classpath` as set by the Maven build process (ie. 
`target/classes`). This test intentionally lives outside of the Kotlin tbDEX
project to test that it may be consumed as built (ie. the distributable JAR)
and contains the native libraries necessary to run.

This acceptance test is run by the CI jobs after building the Kotlin 
distribution to ensure it may be run in all supported environments.

## Building

From this folder, run:

```shell
javac TbdexAcceptanceTest.java \
  -cp ../../bound/kt/target/tbdex-0.0.0-main-SNAPSHOT-jar-with-dependencies.jar
```

You may need to replace the filename of this JAR with the version as built from
Maven (note that this includes the version).

## Running

From this folder, run:

```shell
java -classpath \
  ../../bound/kt/target/tbdex-0.0.0-main-SNAPSHOT-jar-with-dependencies.jar:. \
  TbdexAcceptanceTest
```

You may need to replace the filename of this JAR with the version as built from
Maven (note that this includes the version).

You should see output similar to:

```shell
Successfully loaded shared library for tbdex_uniffi_aarch64_apple_darwin
```