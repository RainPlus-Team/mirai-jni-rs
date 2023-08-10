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
    if (project.hasProperty("workDir")) {
        workingDir = file(project.property("workDir") as String)
    }
}

tasks.named<Jar>("jar") {
    manifest.attributes["Main-Class"] = main
    from(configurations.runtimeClasspath.get().map(::zipTree))
    exclude("META-INF/*.RSA", "META-INF/*.SF", "META-INF/*.DSA")
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
    destinationDirectory = if (project.hasProperty("dev")) {
        file("$projectDir/../build/debug")
    } else {
        file("$projectDir/../build/release")
    }
}

tasks.register<Copy>("copy") {
    include("bot.dll")
    include("bot.so")
    include("bot.pdb")
    if (project.hasProperty("dev")) {
        from("$projectDir/../build/debug/rs/debug")
        into("$projectDir/../build/debug")
    } else {
        from("$projectDir/../build/release/rs/release")
        into("$projectDir/../build/release")
    }
}

kotlin {
    jvmToolchain(8)
}

application {
    mainClass.set(main)
}