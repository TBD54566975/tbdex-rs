plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
    mavenLocal()

    maven {
        name = "tbd-oss-thirdparty"
        url = uri("https://blockxyz.jfrog.io/artifactory/tbd-oss-thirdparty-maven2/")
        mavenContent {
            releasesOnly()
        }
    }
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

    implementation("xyz.block:tbdex:commit-b5c73ab-SNAPSHOT")
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


