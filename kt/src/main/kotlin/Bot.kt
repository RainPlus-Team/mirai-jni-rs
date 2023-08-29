import net.mamoe.mirai.event.Event
import java.io.File
import java.nio.file.Paths
import java.util.*
import kotlin.io.path.exists

class Bot {
    init {
        val props = Properties()
        props.load(Bot::class.java.getResourceAsStream("native.properties"))
        val name = System.mapLibraryName(props["filename"].toString())
        val workDir = System.getProperty("user.dir")
        var libPath = Paths.get(workDir.toString(), name)
        println(libPath)
        if (!libPath.exists()) {
            val codeDir = File(Bot::class.java.protectionDomain.codeSource.location.toURI().path).absolutePath
            libPath = Paths.get(codeDir, name)
        }
        System.load(libPath.toString())
    }
    external fun initialize()

    external fun eventListener(className: String, ptr: Long, bot: net.mamoe.mirai.Bot, event: Event)
}