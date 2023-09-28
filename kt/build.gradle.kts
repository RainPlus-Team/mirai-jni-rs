val nativeName: String
    get() = project.property("native.filename")!!.toString()
val nativePrivate: String
    get() = project.property("native.private")!!.toString()

val asApplication: Boolean
    get() = (project.findProperty("loader.asApplication") ?: "yes") == "yes"
val asPlugin: Boolean
    get() = (project.findProperty("loader.asPlugin") ?: "no") == "yes"

val coreVersion: String
    get() = (project.findProperty("mirai.core.version") ?: "2.15.0").toString()
val consoleVersion: String
    get() = (project.findProperty("mirai.console.version") ?: "2.15.0").toString()

val releaseName = if (rootProject.ext.has("releaseName")) {rootProject.ext.get("releaseName")} else {"debug"}

if (!asApplication && !asPlugin) {
    error("The loader must be configured as application, plugin or both.")
}

plugins {
    kotlin("jvm") version "1.9.0"
    java
    application
}

group = "org.rainplus.mirai.loader"
version = "1.0-SNAPSHOT"

val main = "org.rainplus.mirai.loader.MainKt"

sourceSets {
    main {
        kotlin {
            if (!asPlugin) {
                exclude("org/rainplus/mirai/loader/plugin/**")
            }
        }
    }
}

repositories {
    mavenCentral()
}

dependencies {
    if (!asPlugin) {
        api(platform("net.mamoe:mirai-bom:$coreVersion"))
        api("net.mamoe:mirai-core-api")
    } else {
        implementation(platform("net.mamoe:mirai-bom:$coreVersion"))
        implementation("net.mamoe:mirai-core-api")

        testImplementation("net.mamoe:mirai-console-terminal:$consoleVersion")
    }
    if (asApplication) {
        runtimeOnly("net.mamoe:mirai-core")
        if (asPlugin) {
            compileOnly("net.mamoe:mirai-console")
        }
    } else {
        compileOnly("net.mamoe:mirai-core")
        compileOnly("net.mamoe:mirai-console")
    } // maybe this can be optimized?

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