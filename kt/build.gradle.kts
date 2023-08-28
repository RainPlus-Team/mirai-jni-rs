import org.jetbrains.kotlin.backend.common.push

plugins {
    kotlin("jvm") version "1.9.0"
    java
    application
}

group = "org.rainplus.qbot.ng"
version = "1.0-SNAPSHOT"

val main = "org.rainplus.qbot.ng.MainKt";

repositories {
    mavenCentral()
}

dependencies {
    api(platform("net.mamoe:mirai-bom:2.15.0"))
    api("net.mamoe:mirai-core-api")
    runtimeOnly("net.mamoe:mirai-core")

    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

tasks.named<JavaExec>("run") {
    dependsOn("buildNative")
    standardInput = System.`in`
    if (project.hasProperty("workDir")) {
        workingDir = file(project.property("workDir") as String)
    }
}

tasks.named<Jar>("jar") {
    manifest.attributes["Main-Class"] = main
    from(configurations.runtimeClasspath.get().map(::zipTree))
    exclude("META-INF/*.RSA", "META-INF/*.SF", "META-INF/*.DSA")
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
    destinationDirectory = if (project.hasProperty("release")) {
        file("$projectDir/../build/release")
    } else {
        file("$projectDir/../build/debug")
    }
}

tasks.register("buildNative") {
    outputs.upToDateWhen { false }
    exec {
        workingDir("$projectDir/../rs")
        executable("cargo")
        val list = mutableListOf("build", "--package", "bot", "--target-dir")
        if (project.hasProperty("release")) {
            list.add("$projectDir/../build/release/rs-target")
            list.add("--release")
        } else {
            list.add("$projectDir/../build/debug/rs-target")
        }
        if (project.hasProperty("target")) {
            list.add("--target")
            list.add(project.property("target").toString())
        }
        args(list)
    }
    finalizedBy("copyNative")
}

tasks.register<Copy>("copyNative") {
    include("bot.dll")
    include("bot.so")
    include("bot.pdb")
    val targetDir = if (project.hasProperty("target")) {
        "/" + project.property("target").toString()
    } else {
        ""
    }
    if (project.hasProperty("release")) {
        from("$projectDir/../build/release/rs-target$targetDir/debug")
        into("$projectDir/../build/release")
    } else {
        from("$projectDir/../build/debug/rs-target$targetDir/debug")
        into("$projectDir/../build/debug")
    }
}

kotlin {
    jvmToolchain(8)
}

application {
    mainClass.set(main)
}