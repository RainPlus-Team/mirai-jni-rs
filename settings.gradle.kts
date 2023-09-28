pluginManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}

plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}

rootProject.name = "mirai-jni-rs"
include("kt")
include("rs")

project(":kt").name = "loader"
project(":rs").name = "native"