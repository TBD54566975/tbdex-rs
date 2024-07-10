plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
    mavenLocal()
    maven {
        setUrl("https://jitpack.io")
    }
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("com.sparkjava:spark-core:2.9.4")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.github.TBD54566975:tbdex-rs:v1.0.10")
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


