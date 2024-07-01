plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("com.sparkjava:spark-core:2.9.4")
    implementation("tbdex.sdk.core:tbdex-core-kt:1.0-SNAPSHOT")
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
    mainClass.set("com.example.ApplicationKt")
}

