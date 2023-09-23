val cargoExecutable: String
    get() = project.findProperty("cargo.executable")?.toString() ?: "cargo"
val packageName: String?
    get() = project.findProperty("package.name")?.toString()
val nativeName: String
    get() = project.property("native.filename")!!.toString()

val releaseName = if (rootProject.ext.has("releaseName")) {rootProject.ext.get("releaseName")} else {"debug"}

val buildOutputDir = "${rootProject.buildDir}/$releaseName"
val targetDir = "$buildOutputDir/rs-target"

tasks.register("build") {
    group = "build"
    outputs.upToDateWhen { false } // TODO: proper up to date
    doLast {
        exec {
            workingDir(projectDir)
            executable(cargoExecutable)
            val list = mutableListOf("build", "--target-dir", targetDir)
            if (packageName != null) {
                list.addAll(arrayOf("--package", packageName!!))
            }
            if (project.hasProperty("release")) {
                list.add("--release")
            }
            if (project.hasProperty("target")) {
                list.add("--target")
                list.add(project.property("target").toString())
            }
            args(list)
        }
    }
}

tasks.register("copy") {
    group = "build"
    outputs.upToDateWhen { false } // TODO: proper up to date
    dependsOn("build")
    doLast {
        copy {
            include("bot.dll")
            include("bot.so")
            include("bot.pdb")
            val target = if (project.hasProperty("target")) {
                "/" + project.property("target").toString()
            } else {
                ""
            }
            from("$targetDir$target/$releaseName")
            into(buildOutputDir)
            rename {
                val ext = it.substring(3)
                nativeName + ext
            }
        }
    }
}

// Inject native build tasks as dependencies of loader
val loaderTasks: TaskContainer = rootProject.project(":loader").tasks
loaderTasks.named("run") {
    dependsOn(":native:copy")
}
loaderTasks.named("build") {
    dependsOn(":native:copy")
}