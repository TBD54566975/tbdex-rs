plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
    mavenLocal()

    // Optional: Add JitPack repository if using dependencies hosted on JitPack
    maven { url = uri("https://jitpack.io") }
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("com.sparkjava:spark-core:2.9.4")

    // For local development dependency.
    // Install the dependency locally by running `mvn install` in the `bound/kt` directory
    // implementation("tbdex.sdk.core:tbdex-core-kt:1.0-SNAPSHOT")

    // GitHub dependency
    // Reference: https://github.com/TBD54566975/tbdex-rs/packages/2210202
    // implementation("tbdex.sdk.core:tbdex-core-kt:0.0.1")

    // JitPack dependency
    // Reference: https://jitpack.io/#TBD54566975/tbdex-rs/
    implementation("com.github.TBD54566975:tbdex-rs:v1.0.25")
}

java {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        jvmTarget = "11"
    }
}

application {
    mainClass.set("MainKt")
}


