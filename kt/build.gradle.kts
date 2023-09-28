val nativeName: String
    get() = project.property("native.filename")!!.toString()
val nativePrivate: String
    get() = project.property("native.private")!!.toString()

val releaseName = if (rootProject.ext.has("releaseName")) {rootProject.ext.get("releaseName")} else {"debug"}

plugins {
    kotlin("jvm") version "1.9.0"
    java
    application
}

group = "org.rainplus.mirai.loader"
version = "1.0-SNAPSHOT"

val main = "org.rainplus.mirai.loader.MainKt"

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

tasks.withType<ProcessResources> {
    filesMatching("native.properties") {
        expand(
            "isPrivate" to nativePrivate,
            "filename" to nativeName
        )
    }
}

tasks.named<JavaExec>("run") {
    standardInput = System.`in`
    workingDir = if (project.hasProperty("workDir")) {
        file(project.property("workDir") as String)
    } else {
        file("${rootProject.buildDir}/$releaseName")
    }
}

tasks.named<Jar>("jar") {
    manifest.attributes["Main-Class"] = main
    from(configurations.runtimeClasspath.get().map(::zipTree))
    exclude("META-INF/*.RSA", "META-INF/*.SF", "META-INF/*.DSA")
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
    destinationDirectory = file("${rootProject.buildDir}/$releaseName")
}

kotlin {
    jvmToolchain(8)
}

application {
    mainClass.set(main)
}