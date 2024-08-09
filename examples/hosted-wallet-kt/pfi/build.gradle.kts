plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
    mavenLocal()

    maven {
        name = "tbd-oss-snapshots"
        url = uri("https://blockxyz.jfrog.io/artifactory/tbd-oss-snapshots-maven2/")
        mavenContent {
            snapshotsOnly()
        }
    }
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("com.sparkjava:spark-core:2.9.4")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")

    // For local development dependency.
    // Install the dependency locally by running `mvn install` in the `bound/kt` directory
    // implementation("xyz.block:tbdex:0.0.0-main-SNAPSHOT")

    // For a snapshot from maven
    // Update the short git commit SHA below
    implementation("xyz.block:tbdex:commit-cc93a9c-SNAPSHOT")

    // For the official release on maven central
    // implementation("xyz.block:tbdex:3.0.0")
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


